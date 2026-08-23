# CJ0 ARM CJ-B compare-ready physical port contract

Status: **FROZEN FOR DEVELOPMENT COMPARISON; NO EXECUTION AUTHORIZATION**.

## Ported matter

The comparison unit is the byte-exact candidate module at SHA-256
`ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188`.
Its persistent substrate consists only of ordinary CELL fields, ordinary ARROW
fields, and queued generation-bound SPIKE fields already serialized by the
module. The port adds no side channel or evaluator state.

## Consume contract

When a live source CELL fires, each live outgoing ARROW inspects the decayed
current state of its destination CELL exactly once. Transmission occurs only
when `destination.state + arrow.coupling >= destination.threshold`. On
transmission, the inspected destination state is reset to zero and only that
traversed ARROW receives a finite local return-eligibility window. Suppression
does not create eligibility.

Current destination state is the evidence input. An old learned ARROW firing
from its source without current destination contribution remains below
threshold and cannot manufacture fresh co-participation evidence.

## Produce contract

A successful inspection emits one ordinary SPIKE carrying the summed impulse,
bound to the destination generation and the traversed ARROW generation. A
timely ordinary return can strengthen that same eligible ARROW. A stale queued
SPIKE whose ARROW or destination generation no longer matches performs its
generation check and has no downstream effect.

Cross-region effects remain ordinary ARROW crossings. Recursive learned output
is a normal CELL firing and enters this same consume/produce path without a
level-specific branch.

## Persistence and lifecycle contract

- reusable organization: ARROW `live`, `resistance`, and `coupling`;
- maximum learned candidate coupling in this arm: `2`;
- return window: existing finite local window;
- forgetting: existing unsupported-use and periodic ordinary pressure;
- deallocation: resistance reaches zero, liveness clears, generation advances;
- bootstrap: an externally fired CELL generically proposes local ARROWs to
  nearby live CELLs when no live route already occupies that physical path;
- recovery: only fresh ordinary activity can mature a replacement.

## Comparison invariants

A conforming comparison must preserve candidate bytes or explicitly report a
different law; use fresh identities; match route/activity/timing/pressure/
effect/identity marginals; serialize proposal, delivery, pressure, generation,
return, coupling, resistance, deallocation, crossing, work, storage, replay,
and quiescence; and keep the GATE v1 negative visible.

The mechanically corrected development schedule uses changed-world
within-round offset `2` uniformly for every route order, physical variant, and
timing stratum. The pressure-boundary control at ticks `8 -> 10 -> 11 -> 13 ->
14 -> 16` is part of the comparison contract, not an exception to it.

This document defines a compare-ready port only. CJ-B development ends at
GATE.
