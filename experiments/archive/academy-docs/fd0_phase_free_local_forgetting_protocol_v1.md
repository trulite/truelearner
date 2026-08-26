# FD0 phase-free local forgetting protocol v1

Status: frozen before runtime or evaluator edits.

Parent: stopped PC0 result
`pc0-pressure-closure-compatibility-stopped-negative-v1`.

## Conceptual separation

```text
Participation                  causal continuity only
Modulation x participation     consequence-supported learning
Resistance                     durable structural persistence
Local decay                    ordinary unsupported forgetting
Resource pressure              absent from FD0; reserved for actual scarcity
```

FD0 asks only whether local physical decay can replace the supplied global
ten-tick forgetting epoch. It does not change participation, Modulation,
proposal, capacity, or resource-competition laws and does not run ARC.

## Single candidate law

Retain the predecessor's universal ten-tick material timescale as a decay rate,
not a global phase. Every live ARROW carries a fractional local decay load. For
an elapsed physical duration `dt` during which it is live:

```text
local_decay_load += dt
durable_loss = local_decay_load / 10
local_decay_load %= 10
resistance -= durable_loss
resistance == 0 -> deallocate
```

The arithmetic may compute the interval in one host operation, but the result
must equal every physical tick being applied in sequence. If an ARROW reaches
zero within a large interval, its physical work and liveness end at that local
death age rather than at the host call boundary.

The local load begins at zero whenever a new or re-proposed ARROW becomes live.
No `created_at`, `expires_at`, deadline, age class, or task horizon is stored.
The same law applies to every resistance value and every ARROW.

Traversal and participation do not enter the decay equation. Traversal never
raises resistance or resets local decay load. Qualified Modulation remains the
only tested route to durable gain, but consequence consolidation is reserved
for FD1 rather than claimed here.

The predecessor `pressure_tick` and pressure-phase API may remain dormant for
checkpoint/source compatibility during FD0, but they may not influence local
decay, liveness, resistance, scheduling, or PhysicalWork.

## Matrix

Run all worlds under Reference and Production mechanics, two fresh identity
roots, creation phases `0..9`, and exact same-mechanics replay.

### F0 — phase-free weak lifetime

Create otherwise identical resistance-one candidates at absolute phases
`0..9`. Observe them at equal local ages.

- same age gives identical local decay load, resistance, liveness, work, and
  death age;
- the candidate is live through age 9 and dead at age 10;
- absolute clock phase has no causal effect.

### F1 — resistance scales lifetime

At every creation phase, resistance `1`, `2`, and `4` candidates receive no
activity or Modulation.

- death ages are exactly `10`, `20`, and `40` local ticks;
- intermediate remaining resistance/load is determined only by age and initial
  resistance;
- there are no named weak/medium/strong classes in substrate code.

### F2 — traversal cannot alter forgetting

Create two equal-resistance routes at the same tick:

- route A is repeatedly traversed without Modulation;
- route B is never traversed.

At every equal age, A and B must have identical durable resistance, local decay
load, liveness, and death age. A may carry participation while live, but that
state cannot reset, attenuate, absorb, delay, or otherwise affect decay.

### F3 — host time partition invariance

Advance one copy tick-by-tick and another across large uneven jumps. Require
identical future-causal state, PhysicalWork, death age, generation, stale
blocking, and subsequent behavior.

### F4 — no scarcity semantics

Repeat F0-F3 in an arena with abundant unused ARROW capacity. Allocation count
and free capacity may not affect local decay. FD0 introduces no reclamation or
competition rule.

## Comparison

Reference and Production must agree exactly on ordered physical transitions,
deliveries, resistance/liveness/generation, local decay load, participation,
proposals, deallocations, pending activity, physical clock, canonical durable
body, PhysicalWork, natural quiescence, and replay.

Across creation phases, compare state by local age while separately serializing
absolute tick. Exact checkpoint bytes need not match between different absolute
times; restored continuation from each checkpoint must preserve its own future
exactly.

## Static prohibitions

FD0 substrate logic may contain no:

- global `tick % period` forgetting trigger or absolute forgetting epoch;
- per-ARROW expiry/deadline/TTL/created timestamp;
- read of participation, plastic support, Modulation timing, ARC state, or
  evaluator input in local decay;
- traversal-based resistance gain or decay-load reset;
- capacity/scarcity branch or resource-pressure rule;
- eligibility restoration, pressure shield, or event reordering.

## Decision

Any matrix, mechanics, replay, quiescence, work, stale-generation, or static
failure is an immutable FD0 negative. A complete pass establishes FD0
development readiness only.

FD1, removal of dormant predecessor pressure fields/APIs, RC0, ARC, authority,
oracle, and `arch.md` remain blocked.
