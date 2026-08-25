# PQLC1 depth composition protocol v1

Status: frozen before any PQLC1 evaluator or Rust change.

Parent: PQLC0 development-positive result at
`acb7a47a0b39f6fb10b5f6eb6bdaf84715e30d0d`, tagged
`pqlc0-participation-qualified-local-closure-result-v1`.

## Question

Can the exact frozen PQLC0 local rule compose through contact depths
`1, 2, 4, 8, 16` without adding any depth-dependent runtime mechanism?

PQLC1 proposes no new substrate law. The PQLC0 core source, trigger/effect
factorization, CPC1 participation dynamics, pressure behavior, and Production
mechanics must remain byte-identical.

## Chain geometry

A depth-`D` positive fixture contains ordinary contact compartments
`C1..CD`. The forward episode physically traverses every contact's outgoing
Drive structure. One downstream Modulatory consequence then reaches `CD`.

Continuation may occur only by repetition of the frozen local rule:

```text
Modulatory arrival at Ci
        +
remaining local participation at Ci
        ↓
explicit QLP ARROW Ci -> C(i-1) traverses
        ↓
ordinary Modulatory arrival at C(i-1)
```

`C1` has no preceding QLP edge. At every level the CELL does not fire because
of closure. A depth-`D` complete chain therefore expects `D - 1` QLP
traversals and support at all `D` participating contact structures.

Fixture construction may use a host-side loop over the preregistered depths.
The runtime may not receive, inspect, store, or branch on depth.

## Frozen case families

### Complete

At each depth `1, 2, 4, 8, 16`, traverse the complete forward chain, introduce
one consequence at the final contact, and require:

- support at all `D` contact structures;
- exactly `D - 1` QLP traversals;
- natural quiescence and exact replay.

Record QLP traversals and PhysicalWork by depth. Their growth is diagnostic,
not a tuned acceptance threshold.

### Structural participation break

Keep the ordinary topology but prevent one live contact from firing/traversing
its outgoing Drive structure. Independently start the physically separate
downstream suffix so it genuinely participates. Closure must affect the
downstream suffix and stop at the first unqualified contact; the break and
upstream prefix receive zero support.

Frozen break indices, counted from `C1 = 0`:

```text
depth 1   0
depth 2   0, 1
depth 4   0, 2, 3
depth 8   0, 4, 7
depth 16  0, 8, 15
```

### Temporal participation break

Traverse the whole topology, but place one ordinary long forward delay after
the selected contact so its CPC1 participation and all earlier participation
naturally relax to zero before the downstream suffix and consequence occur.
The selected contact is still live and was actually traversed. Closure must
support only the still-participating downstream suffix and stop at the expired
contact.

Use the same frozen break indices as the structural family. The long delay is
`1024` physical ticks, inherited from the accepted CPC1 zero-participation
control; it is characterization, not a selected credit horizon.

### Wrong branch

At depths `2, 4, 8, 16`, a qualifying final compartment has explicit QLP
topology toward two adjacent upstream branches. Only branch A participates.
The final compartment may physically emit toward both; branch A may continue,
while branch B must stop at its first unqualified contact and receive zero
support.

### Honest fan-out

At depth `4`, both explicit adjacent branches genuinely participate. Both may
continue and receive support. No one-path or winner semantics is required.

### Recurrent closure

At every depth `1, 2, 4, 8, 16`, add one QLP edge from `C1` back to `CD`, making
the closure topology recurrent. All QLP edges have physical delay one. After
one legitimate consequence, the unmodified system must naturally quiesce
within `8192` physical operations.

No attenuation, trace consumption, TTL, refractory state, cycle detection, or
special stop may be added. Timeout, ceiling exhaustion, or non-quiescence is an
immutable negative.

## Unconditional matrix

The case inventory is fixed:

```text
complete                    5
structural break           12
temporal break             12
wrong branch                4
honest fan-out              1
recurrent closure           5
                           --
case variants              39
```

Run every variant across:

- two fresh identity roots;
- pressure phases `0..9`;
- Reference and Production mechanics;
- exact same-mechanics reconstruction.

This yields `39 * 2 * 10 = 780` physical cases and `1560` mechanics rows,
before `3120` reconstruction runs are counted.

For every physical case, compare exactly before evaluating its predicate:

- ordered physical transitions;
- QLP traversals;
- Drive and Modulatory deliveries;
- per-contact participation and plastic support;
- proposals, deallocations, and PhysicalWork;
- clock and pressure phase;
- canonical durable body and explicit trigger surface;
- natural quiescence and exact replay.

No sorting, trace normalization, checkpoint weakening, comparator repair, or
post-observation fixture change is authorized.

## Static prohibitions

The frozen PQLC0 substrate files must remain byte-identical. No runtime logic
may add or encode equivalents of:

```text
depth counter
recursive depth parameter
path list
predecessor or parent pointer
backward traversal mode
continue_credit
start or terminal marker
hop count
route identity
```

No new physical effect, trigger, pressure rule, participation arithmetic,
damping law, or eligibility behavior is allowed. Depth and break position may
exist only in evaluator fixture construction and serialized evidence.

## Decision

- Any physical predicate, cycle, quiescence, replay, representation, hash, or
  static-audit failure: PQLC1 stopped negative; freeze and stop.
- All `780` physical cases pass: PQLC1 development positive for unchanged
  local-rule composition through tested depths `1..16`.

No pressure integration, eligibility deletion, ARC A3-A5, authority, oracle,
or `arch.md` change occurs in PQLC1.
