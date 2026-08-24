# LR0-D0 qualified physical return information-sufficiency audit v1

Status: **STATIC DIAGNOSTIC COMPLETE; LR0 SUCCESSOR REQUIRED; NO EVIDENCE SPENT**.

Start: PX3-R6 integrated preflight-collapse commit `ad3fc01`, tag
`px3-r6-integrated-preflight-collapse-v1`.

## Question

> Does authoritative PX0 expose enough physical information at its local
> plasticity boundary to distinguish renewed forward drive from genuinely
> completed downstream return without a semantic RETURN, CREDIT or CAUSE bit?

This audit diagnoses two candidate physical routes without implementing either:

1. **topological separation** -- forward drive and return reach distinct local
   ports/cells;
2. **traversal-qualified return** -- the learning update can inspect the actual
   incoming physical traversal and its recent participation relation.

## Frozen inputs

| input | SHA-256 |
|---|---|
| authoritative PX0 law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| frozen R6 source | `12d9422cc43d43a88da9d8046a2ab7fbdc8f9447236e97bc73483d0d4ce7eb4f` |
| frozen R6 CSV | `35b68303630f69c326fadad1ccc988e807ae0e1a77703b4e751732e0cdeae4d8` |
| R6 integration-collapse audit | `52072394fcf9867f23d1ec982f030fac6d5b5601c8f7294f178098743664b033` |

The final hash is recorded from the parent commit's immutable file. No source
or artifact was executed or altered for D0.

## Authoritative information flow

At delivery, PX0's private `Spike` still contains:

```text
arrival tick
origin physical id
target CellId
impulse
incoming ArrowId + generation, or external-arrival absence
```

The propagation loop validates the incoming ArrowId and generation, then calls:

```text
apply_local_return(spike.target, tick, work)
```

before accumulating state or testing whether the target fires. The incoming
ArrowId, its source cell, `origin_physical`, impulse and external/internal
status are not passed to plasticity.

`apply_local_return` receives only target cell and tick. It strengthens every
live eligible arrow whose `from` equals that cell. Therefore all accepted
arrivals at one cell are plasticity-equivalent, even when their physical paths
are different.

## Hypothesis A -- topological separation

Pure topology cannot express qualified return under the unchanged PX0 update:

```text
forward and return both target P
  -> apply_local_return sees the same (P, tick) surface

forward targets F, candidate source remains P
  -> F must cause P to execute through an F->P arrival
  -> that arrival targets P and is again treated as return

candidate source moves to F, return remains at P
  -> apply_local_return(P) cannot update an arrow whose source is F

return is routed to F so it can update F's candidate
  -> renewed forward drive and return collide at F instead
```

Authoritative cells have no separate input ports or compartments, and local
return cannot update an eligible arrow at a neighboring source. Consequently,
cell rearrangement merely moves the ambiguity. A genuine port-separated
solution would require a successor local interaction that relates a physical
return port to a different execution source; that interaction is not in PX0.

**D0-A classification: unavailable under the authoritative law.**

## Hypothesis B -- traversal-qualified return

The propagation layer retains the incoming ArrowId and generation until the
line immediately before plasticity. Thus physical route identity is available
in the substrate but discarded at the learning boundary.

Passing an incoming traversal to a successor plasticity function is
representationally possible without a semantic return bit. However, merely
requiring that *some* incoming arrow traversed is insufficient: renewed
opportunity drive also traverses an ordinary incoming arrow. A later LR0
mechanism must preregister a content-neutral physical qualifier relating the
incoming traversal to the recently traversed outgoing candidate or to a live
completed-loop trace such as R6. The evaluator may not nominate an ArrowId as
"the return arrow."

**D0-B classification: physically representable only through a successor law;
the current learning interface discards the required route information.**

## Minimum honest successor boundary

LR0 cannot be implemented as downstream PX3 attribution alone. The smallest
honest change surface is the base local-plasticity decision itself:

```text
old PX0:
  eligible outgoing arrow + any arrival at its source -> strengthen

LR0 successor candidate:
  eligible outgoing arrow
  + physically qualified incoming traversal/trace at its source
  -> strengthen
```

This statement does not choose the qualifier. In particular, D0 does not
authorize:

- a semantic RETURN/CREDIT/CAUSE flag;
- a harness-selected return ArrowId;
- a region, phase, position or amplitude value chosen merely to encode return;
- a timeout, since R4 established overlapping forward/return windows;
- another downstream detector that leaves generic PX0 updates active first.

## Required LR0 development discriminator

Any separately preregistered successor must conjunctively establish:

```text
candidate traversal + renewed upstream drive + no world return
  -> source may execute
  -> candidate resistance unchanged

candidate traversal + completed downstream path + real return
  -> exactly one strengthening

both arrivals coincide
  -> exactly one strengthening, not two

upstream drive alone
  -> may execute source
  -> no candidate strengthening

return path independently active without candidate participation
  -> no candidate strengthening
```

It must serialize incoming physical ArrowIds/routes, candidate eligibility,
actual source executions, resistance transitions and exact update counts. A
resistance proxy alone is insufficient.

## Stack boundary

PX0 remains authoritative for its frozen claim and worlds. D0 establishes a
new compositional boundary: its generic local-return law is insufficient when
forward and feedback activity can reach the same source in continuous time.

If LR0 changes the base law, the continuous stack becomes a successor stack.
Before PX3 reopens, authoritative PX0 worlds and frozen PX1/PX2 behavior need
separate conformance replay against that successor. D0 grants no PX3 authority
and leaves PX4 blocked.
