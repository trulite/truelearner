# RS1 inhibitory topology sufficiency result audit v1

Status: development positive; ordinary inhibitory topology is sufficient for
the frozen recurrent-stability discriminator.

## Execution and integrity

The frozen RS1 matrix executed exactly once in fresh E2B worker
`iekh83ae3qf36j9v2tvye` from evidence-eligible commit `80cf906`.

```text
physical cases                         440 / 440 PASS
mechanics rows                         880 / 880 PASS
same-mechanics exact replay            880 / 880 PASS
Reference / Production physical match  880 / 880 PASS
frozen predicates                      880 / 880 PASS
absolute-phase classification          invariant
maximum PhysicalWork                   288
classification                         RS1 positive
```

Evidence hashes:

```text
matrix  a83e36b6f26287391950ad395374fd72589e491fd3ef3d00ed7d56be1a627953
report  5591ce12f685eb342e5475c7d776ae707381185592738e7790ec4294d15f9b6c
```

No Modulation, plasticity update, QLP traversal, structural proposal,
deallocation, coupling change, new physical type, or new physical law
occurred. All retained ARROWs remained live with their frozen coupling and
resistance above 999,000.

## Sufficiency result

The uninhibited coupling-two/threshold-two reciprocal loop reproduced RS0:
exactly periodic, period two firings over two ticks, and active through both
observation ceilings.

Adding ordinary local feedback through threshold-one CELL relays and H16
negative Drive made every frozen executable recurrent family naturally
quiescent:

- reciprocal delays 0+1, 1+1, 2+2, and 3+3;
- cycles of lengths 2, 3, 4, and 8;
- executable coupling/threshold pairs 1/1, 2/2, and 3/3;
- the alternating-phase main geometry.

Every excitatory CELL in these worlds fired exactly once. Every local relay
fired exactly once and delivered one negative Drive. The main reciprocal world
used PhysicalWork 7; the length-eight recurrent world used 25.

Both inhibited one-way chains preserved the intended causal execution: all
eight excitatory CELLs fired exactly once, with seven forward traversals, then
the bodies settled. This held for excitatory delay zero and delay one.

## Locality and necessity controls

The same H16 magnitude did not stabilize the target loop when its physical
effect was absent or misplaced:

- disconnected feedback delivered 96 negative Drives to unrelated sink CELLs
  while the target loop remained exactly periodic;
- untraversed X-to-I topology delivered no negative Drive and the target loop
  remained exactly periodic;
- with two simultaneous reciprocal loops, only the inhibited neighborhood
  settled (`1|1` firings), while the separate uninhibited loop remained
  periodic (`141|140` observed firings).

The frozen strength characterization showed a mechanical boundary on the
coupling-two/threshold-two reciprocal geometry:

```text
H1  periodic
H2  periodic
H3  quiescent
H4  quiescent
H8  quiescent
H16 quiescent (the reciprocal-delay-1 H16 family)
```

No magnitude was selected after observation; H16 remained the preregistered
sufficiency candidate.

## Classification

RS1 is positive. Existing signed Drive, ordinary CELL thresholds, ordinary
ARROW topology, refractory state, and decay can express a stable executable
recurrent body. Stabilization is spatially local and does not block the first
intended acyclic traversal.

This result does **not** show that the organism can propose, retain, or learn
the required inhibitory topology. It shows only that no transmission-fatigue,
depletion, adaptation, or other new activity-limiting substrate state is
currently justified. The next scientific question is whether stabilizing
topology can arise without being supplied.
