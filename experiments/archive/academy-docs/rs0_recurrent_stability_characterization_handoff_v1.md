# RS0 recurrent stability characterization handoff v1

Status: complete development characterization; stopped before candidate law.

## Result

```text
one-way executable topology          quiescent
acyclic chains                       quiescent
subthreshold recurrent topology      quiescent
same-tick recurrent closure          refractory termination

executable recurrence
+ any positive total cycle delay
→ exactly periodic persistent activity
```

This holds for coupling 1 at threshold 1, coupling 2 at thresholds 1/2,
reciprocal delays 0+1, 1+1, 2+2, and 3+3, alternating phases, and cycles of
lengths 2/3/4/8. The body partition, mechanics representation, absolute clock
phase, and replay do not change the result.

## Frozen lineage

- protocol: `be3b4c8`, tag
  `rs0-recurrent-stability-characterization-protocol-v1`;
- initial implementation: `46a20a5`;
- first freeze: `6f6c6cb`, tag
  `rs0-recurrent-stability-characterization-frozen-v1`;
- observer-control freeze: `787d2b5`, tag
  `rs0-recurrent-stability-characterization-frozen-v1-corrected`;
- corrected physical-equivalence boundary: `b92bc6b`, tag
  `rs0-recurrent-stability-characterization-evidence-eligible-v1`;
- successful preflight: `6375a30`, tag
  `rs0-recurrent-stability-characterization-preflight-v1`;
- immutable characterization: `bcb204a`, tag
  `rs0-recurrent-stability-characterization-positive-v1`.

## E2B provenance

- reusable implementation/preflight worker: `i8mm34sawk38wa16yua5o`;
- sole fresh characterization worker: `ings3j8djoetx2ququjeg`;
- formatting, strict Clippy, static audit, observer pause/resume control: PASS;
- matrix, replay, and Reference/Production equivalence: PASS.

## Next boundary

RS0 independently establishes the missing problem class:

> Existing local physics has no ordinary activity-limiting process once a
> recurrent loop is executable outside same-tick refractoriness.

A fresh successor may now test one candidate such as transient transmission
depletion. It must be preregistered independently and must not be introduced as
an RS0 repair. FD2, ARC, authority, oracle, and `arch.md` remain unchanged.

