# PX1-PT1 attributed-margin stability MICRO protocol

Status: **PREREGISTERED; MICRO EVIDENCE UNSPENT; PX1 NON-AUTHORITATIVE**.

## Frozen lineage

- authoritative PX0 source SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- PT0 implementation SHA-256:
  `f0b754ed6f7b0603668319a0735da91b4c168f909d4024fd5ce5e2aea4197410`;
- immutable PT1 reserve-2 negative audit SHA-256:
  `53ebee871ff068e222fd5a9049203b59e94a67acf8935c96b5800d9f467d417b`;
- frozen positive reserve-3 PROBE implementation SHA-256:
  `1bcfe295fb8989d1c6489e7c255b128912b98afef8b550eea15b5eaf0e06b443`;
- positive PROBE CSV SHA-256:
  `cda4bf6750abb40f7b3798e84c0b6f39527704a02c69b133896ce51c3925420b`;
- positive PROBE audit SHA-256:
  `5ffe23cfa570a34a228f1f91c99b91d16bc92279ce295c0d3f9846dc80b04146`.

The reserve `3`, margin threshold `4`, physical topology, thresholds, delays,
couplings, PX0 acquisition law, effect-triggered coincidence cells, and global
return topology remain unchanged.

## Fresh MICRO worlds

Each world runs a primary layout and a fresh mirrored/reversed-allocation
transfer layout:

1. **support A** — only A is physically supported during development;
2. **support B** — only B is physically supported;
3. **no support** — neither branch completes a continuation;
4. **blocked global return** — A completes, but the ordinary global-return edge
   is physically absent;
5. **return without effect** — neither outlet fires, while an identical global
   return arrives externally at both trace cells through the hub;
6. **joint participation** — both branches and outlets genuinely participate.

No world contains an evaluator-selected local return. Support and return are
ordinary anonymous physical activity/topology.

## Independently serialized chain

For A and B separately, during development and held-out execution record:

1. branch firing;
2. outlet firing;
3. trace-cell arrivals;
4. trace-cell firing;
5. local branch-return arrival;
6. continuation resistance;
7. outward held-out execution.

Also record correspondence resistance, training/held-out/post-gap source
refiring, quiescence, work, complete fingerprints, and exact duplicate replay.

The central support-A/B cells require during held-out execution:

```text
both branches fire
only mature outlet fires
same global return reaches both trace cells
only effect-bearing trace cell fires
only its branch receives local return
only its continuation executes outward
```

## Expected physical outcomes

- support A: mature/executable `[true,false]`;
- support B: mature/executable `[false,true]`;
- no support: `[false,false]`;
- blocked return: `[false,false]` despite A branch/outlet training activity;
- return without effect: `[false,false]` and zero trace-cell firing;
- joint participation: `[true,true]`.

All worlds must remain naturally quiescent with zero autonomous source
refiring. Joint participation is allowed to credit both branches; exclusivity
is not supplied.

## Pass and stopping rules

All twelve primary/transfer cells and every independently serialized clause
must pass. A compound behavioral success cannot hide a failed physical stage.

A positive MICRO makes a six-world GATE eligible. A negative freezes exactly
where the chain first breaks. No definitive execution or PX1 authority is
authorized.

