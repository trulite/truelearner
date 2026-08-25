# RS2 learned inhibitory topology consolidated immutable negative v1

Status: complete frozen negative. CE1, FD2 v2, and frozen ARC A2 did not run.

Protocol: `2839ef25dbf3a564ede4ae4d30195d3748a2d498`
(`rs2-learned-inhibitory-topology-consolidated-protocol-v1`).

Frozen evaluator: `acf396b704b28c24bea42f013797773bfbfcfe5b`
(`rs2-learned-inhibitory-topology-consolidated-frozen-v1`).

One-shot fresh E2B worker: `ive857ni4zohqwmudzrcy`.

## Exact stop

The complete consolidated RS2 command executed exactly once. The release
build completed, but the evaluator stopped before publishing its first matrix
row at the native live-checkpoint continuation restore:

```text
src/main.rs:772
PlasticSubstrate::from_live_checkpoint_with_mechanics(...).unwrap()

Err: InvalidPhysicalBody
```

The downloaded evidence archive contains only the empty preregistered result
directory. No CSV or report was published.

## Frozen-source classification

The failure is a cumulative J0/checkpoint representation incompatibility, not
an RS2 learned-inhibition result and not a WS0 causal-wave failure.

Under J0, an orphan junction is deallocated by:

```text
CELL.live       -> false
CELL.generation -> next generation
```

Its stored `resistance` is deliberately not the cause of its lifetime and is
left nonzero. This is consistent with the retained J0 rule that junctions
compute while live incident links determine whether they remain required.

The durable checkpoint body serializes that dead junction unchanged. The
historical body loader then rejects every durable CELL for which:

```text
live != (resistance > 0)
```

Therefore a valid J0-deallocated junction (`live=false`, nonzero dormant
resistance) cannot round-trip through the current native live-checkpoint
loader. RS2 reaches this only when its unsupported generated contact becomes
orphan before the frozen continuation check.

No evidence was rerun and no checkpoint validator, J0 law, RS2 fixture, or
measurement boundary was changed after observation.

## What this result does and does not say

This result establishes only:

> The native checkpoint representation still assumes that CELL liveness is
> equivalent to positive CELL resistance, while retained J0 defines junction
> liveness from incident topology.

It does not establish whether signed consequence selection, learned
inhibition, recurrence stabilization, identity permutation, replay, or
Reference/Production equivalence passes or fails. No complete RS2 case
reached serialization.

WS0 remains development-ready and its full retained prefix remains positive.
The organism runtime stayed byte-identical during the consolidated RS2 gate.

## Frozen hashes

- canonical runtime:
  `d12b02bbb85645a916a5690d5ce5ebfd8e5c9d6820025a0c6d315a55aa0180a9`;
- evaluator:
  `0fa931103ba2a478c6c8a4e7a15dcd6b877a6c566f5e2135172741af6a595663`;
- evaluator manifest:
  `3b09fceafb20f0052fedf74dc3585b6a2dcaad8a615918fdc2d50c5b58ce7b16`;
- protocol:
  `c2212ef6d0d09d54f159c1dfcc3be08b3d8e76f4d2b882a438cb523c950dec20`.

Authority, oracle status, `arch.md`, and the Academy curriculum remain
unchanged.
