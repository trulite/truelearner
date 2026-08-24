# PX1-PT1 attributed-margin stability MICRO v1 negative audit

Outcome: **NEGATIVE (`8/12`)**. This development result is immutable. It was
executed once from the frozen implementation; there was no rerun or rescue.

## Frozen evidence

- implementation commit: `df98d91`;
- implementation source SHA-256:
  `8b094e5ac9dca5c41baf20bb2791da1bcdd0406fbfeb1946dc03d240c4ad0c38`;
- CSV SHA-256:
  `d32e174f77c2440c52baf8978370ff684ddfffb45a72938cbc8cdbb7099f93c2`;
- report SHA-256:
  `0fd36f47902bc9e8bb5ff8abfb960cd3281ffe3a7a262fdd61076bc432e2e731`;
- evidence marker count: `1`;
- process exit: `0`;
- duplicate-exact cells: `12/12`;
- naturally quiescent cells: `12/12` in training, held-out, and post-gap;
- autonomous source refiring: `0` in every phase and cell.

## Passed physical claims

Both primary and fresh mirrored/reversed-allocation cells passed for support A,
support B, no support, and return without effect. In support A/B held-out
execution, both branches fired, only the mature outlet fired, the same global
return reached both trace areas, only the effect-bearing trace fired, only its
branch received local return, and only that continuation crossed outward.

Joint participation physically matured both continuations to resistance
`[17,17]`; both branches, outlets, trace cells, local returns, outward effects,
and post-gap effects were `[1,1]` in held-out execution.

## First failed predicates

The four failed cells form two mirrored measurement boundaries, not a change in
the learning mechanism:

1. **blocked return** — the implementation demanded eight outlet firings and
   eight direct trace arrivals. The frozen physical result was branch firing
   `[8,0]`, outlet firing `[1,0]`, direct trace arrival `[1,0]`, trace firing
   `[0,0]`, local return `[0,0]`, final resistance `[0,0]`, and held-out effect
   `[0,0]`. The weak continuation participated once and then lawfully died when
   no global return supplied local coincidence. The protocol required actual
   branch/outlet training activity, not eight completed outlets.
2. **joint participation** — the implementation predicted three held-out trace
   arrivals per side by counting one global return per mature outlet. Both
   outlets reached the same threshold-1 hub in one physical propagation. The
   hub fired once, so each trace received one direct arrival and one shared
   global arrival: `[2,2]`. Both traces fired and both continuations executed.

The v1 implementation and artifacts remain frozen. Any successor must be
separately preregistered and may correct only these measurement predicates; it
may not alter reserve, topology, thresholds, delays, couplings, learning law,
worlds, namespaces, exposures, or execution schedule.

PX0 remains authoritative. PX1 remains non-authoritative. GATE and definitive
execution remain unauthorized.
