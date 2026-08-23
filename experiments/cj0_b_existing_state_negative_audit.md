# CJ0 ARM CJ-B existing-state sufficiency audit

Status: **FROZEN SOURCE-LEVEL NEGATIVE; NO PHYSICAL CELL EXECUTED**.

## Frozen start

- authoritative commit and tag:
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`,
  `px2-physical-causal-direction-authoritative`;
- authoritative PX0--PX2 law:
  `crates/px0-physical-correspondence/src/lib.rs`, SHA-256
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- frozen PX3 Class-D result: tag
  `px3-physical-event-boundaries-frozen-negative-handoff-v1`;
- immutable PX3-R negatives: tags
  `px3-r-direct-trace-coupling-frozen-negative-handoff-v1`,
  `px3-r-shared-cell-frozen-negative-handoff-v1`, and
  `px3-r-c-downstream-convergence-frozen-negative-handoff-v1`.

The worktree was clean and HEAD and the authoritative tag both resolved to the
exact commit above before this audit was written.

## Exact source result

Existing CELL state can retain a transient local physical contribution, but
existing ARROW transmission cannot use that state as a gate. In the
authoritative `propagate` loop every live outgoing ARROW unconditionally emits
its coupling when its source CELL fires. Destination CELL state is read only
later, when the emitted SPIKE arrives.

Consequently the existing state has only two relevant encodings:

1. ordinary direct trace coupling, already frozen negative because a mature
   ARROW transmits when its source alone fires; or
2. ordinary threshold convergence, already frozen negative because either a
   mature incident coupling becomes singleton-sufficient or a deallocated
   coupling-2 opportunity is recreated at coupling 1 and cannot bootstrap a
   threshold-4 convergence CELL.

Those are PX3-R Arms A and B. They are not rerun, reinterpreted, or rescued.
Arm C's downstream convergence is also not imported; its old organization
recruits crossed structure during reversal.

## Classification and first missing edge

Existing authoritative state is **insufficient** for CJ-B. The first missing
physical edge is a local transmission condition that consumes current
destination CELL state at the moment a weak ARROW attempts transmission.

No new persistent variable is shown necessary. The candidate may use only the
already-existing numeric CELL state, threshold, ARROW coupling, local time,
and existing structural resistance/lifetime fields. Any need for a gate flag,
owner, relation key, contributor identity, or other persistent field is a
mandatory stop.

This audit spends no PROBE, MICRO, GATE, definitive, PX3, or PX-C evidence.

