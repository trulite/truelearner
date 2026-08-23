# PX2 physical causal direction trace-sufficiency implementation audit

Status: **IMPLEMENTATION FROZEN; PROBE EVIDENCE UNSPENT; PX2 NON-AUTHORITATIVE**.

## Frozen source

- implementation:
  `crates/px0-physical-correspondence/examples/px2_physical_causal_direction.rs`;
- implementation SHA-256:
  `e0fc541ad12aa71b2e7adc7d61af6bb1de1355548a73259280d1b25433068e42`;
- protocol SHA-256:
  `5c43ffda226a125bbbaf7f24dbb1ec8e70861b78a2d730c630535254def85c23`;
- authoritative PX0 source SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

No substrate source changed. The implementation composes the authoritative
PX0 acquisition path and authoritative PX1 participation-trace/ordinary-return
path without adding state or a plasticity rule.

## Central discrimination

Forward and reverse worlds make both consequence areas fire at the same tick,
make both trace cells fire, deliver the same shared global return, and deliver
local return to both continuation sources. One consequence was reached through
actual candidate traversal; the other was produced by an ordinary independent
physical perturbation. Only traversal leaves the candidate arrow eligible for
the later local return.

Correlation-only, joint-participation, and blocked-return worlds isolate the
remaining physical dependencies. All stages are serialized independently.

Scenario and direction names exist only in evaluator-side rows. The substrate
contains only cells, arrows, spikes, timing, thresholds, coupling, return,
pressure, and ordinary local plasticity. It contains no cause/direction field,
typed provenance, selected path, role object, or old M1/M2 representation.

## Pre-evidence validation

- formatting: pass;
- focused compilation: pass;
- strict focused Clippy: pass;
- focused substrate test: `1/1` pass;
- non-PROBE refusal: exit `2` before the harness;
- write-once result paths: absent;
- authoritative PX0/PX1 hashes: exact;
- worktree diff: new PX2 evaluator/runner and protocol/audit only.

The only authorized command is the single write-once `--probe` development
surface. MICRO, GATE, definitive execution, PX2 authority, and PX3 remain
unauthorized.
