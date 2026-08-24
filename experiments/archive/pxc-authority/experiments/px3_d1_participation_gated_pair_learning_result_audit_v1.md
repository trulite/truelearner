# PX3-D1 participation-gated pair learning result audit v1

Status: **D1-A POSITIVE; D1-R NEGATIVE; EVIDENCE SPENT; D2 UNIMPLEMENTED**.

## Write-once execution

- frozen implementation commit/tag:
  `d7ee2e7dae3d01909b84b987e5f0aa1007fc0945` /
  `px3-d1-participation-gated-pair-learning-implementation-v1`;
- persistent E2B sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- evidence marker observed exactly once:
  `PX3_D1_PARTICIPATION_GATED_PAIR_LEARNING_EVIDENCE`;
- CSV SHA-256:
  `511efc76fc36c9c3c77815f1bd53bd4f8ea38411f8892bbc26005af1e7d0fecb`;
- Markdown SHA-256:
  `126dc544d2a2782cde6789d44ee2ff4b9ad0fa8c01f2fc7dd9502aeb238ca84b`;
- rows: `26` in exact frozen order;
- D1 core rows: `24`, all passed;
- D1-R rows: `2`, neither positive;
- exact replay: `26/26`;
- natural quiescence: `26/26`;
- generic structural proposals: `0`;
- native work: `27,070` operations;
- staging remnants: absent.

No implementation change or rerun occurred. PX0--PX2 and the spent PX3 negative
remained byte-exact. Candidate formation, reproposal, D2, MICRO and GATE were
not executed.

## D1 core result: positive participation gating

The six candidates began live at resistance `1` with no prior traversal and no
eligibility. The two seeds reproduced exactly.

### Untraversed candidates cannot learn

The `return-only` control physically broadcast one ordinary return from the
shared hub to all six opportunity sources:

```text
candidate traversals:       0|0|0|0|0|0
return arrivals:            1|1|1|1|1|1
resistance before return:   1|1|1|1|1|1
resistance after return:    1|1|1|1|1|1
local return updates:       0
```

A alone, A(4), two scheduled A pulses and late A/B also produced no pair
firing, no candidate traversal and no candidate resistance change. All dormant
candidates deallocated under the fixed pressure gap.

Thus candidate existence and nearby/input/return activity were insufficient.
The candidate itself had to traverse before later activity could strengthen
it.

### One presentation is experienced but not reusable

One simultaneous A+B exposure produced exactly one AB opportunity firing, one
AB candidate traversal carrying native impulse `1`, one consequence and one
shared return:

```text
AB resistance: 1 -> 4 -> 0 at tick 50
held-out AB consequence: 0
```

Every crossed candidate remained at resistance `1` before pressure and then
deallocated. A genuine event occurred, but one presentation did not become
reusable.

### Recurrence earns persistence

Every recurrent A+B row, across raw couplings `1+1`, `2+1` and `4+4`, followed
the preregistered native trajectory:

```text
initial                 1
first traversal+return  4
pressure boundary       3
second traversal+return 6
tick-50 gap             2, live
```

Candidate crossing-impulse sum was `3`: first traversal carried `1`, and the
second native traversal carried `2` after legitimate plasticity. After the gap,
trained AB produced exactly one consequence while crossed AD, gapped A/B and
singleton A produced zero. Unused candidates were absent.

AB traversal with consequence return blocked remained resistance `1` and
deallocated, establishing that traversal opens eligibility but does not itself
provide support.

These results establish the narrow D1-A claim:

> Given symmetric weak candidates, unchanged PX0 traversal/eligibility/return
> physics prevents untraversed candidates from learning and lets recurrent
> traversed candidates earn reuse.

They do not establish autonomous candidate formation, reproposal, reversal,
return provenance or full PX3 organization.

## D1-R result: negative return provenance

The provenance discriminator blocked every consequence return and serialized:

```text
tick 3: AB opportunity fires
        AB candidate traverses once with impulse 1
        AB becomes eligible
tick 4: late A source actually fires
tick 5: late A produces a second authoritative A trace
        the A trace reaches AB while eligibility is live
consequence return arrivals: 0|0|0|0|0|0
AB resistance: 1 -> 4
```

The same result occurred in both seeds. The late upstream trace therefore
acted as local return to the eligible AB source even though the physical
consequence-return path was absent.

This does not contradict D1 core: AB really had participated, so eligibility
was legitimately open. It shows that unchanged target-local return uses timing
and target identity but not the provenance or direction of the later arrival.

Freeze the separate conclusion:

```text
participation attribution: solved for D1
return provenance:         unsolved for D1-R
```

No coupling value is inferred from resistance. The authoritative observations
are the initial candidate crossing impulse `1`, absence of all consequence
return crossings, and native resistance change `1 -> 4` after the late A trace.

## Classification and handoff

- D1 core classification: **D1-A positive**.
- Provenance classification: **D1-R negative**.

A separately preregistered D2 normalization diagnostic remains scientifically
independent and may ask whether mature AB execution becomes exactly one ordinary
PX1 participation trace. This result does not implement or execute D2.

Any attempt to claim consequence-specific evidence must first address the
frozen D1-R collapse without weakening the positive participation-gating result
or adding semantic return labels. Full PX3 lifecycle and authority remain
absent.
