# PD1 local pressure-participation protocol v1

Status: frozen before any PD1 substrate or evaluator change.

Parent: PD0 characterization result at
`03f2eed`, tagged `pd0-old-pressure-interaction-result-v1`.

## Question

Can ordinary pressure interact continuously with the same path-local
participation state already used by CPC1 and PQLC, so recently used weak
structure remains provisionally viable without consulting a rectangular
eligibility-pressure exception or treating use as durable learning?

PD1 changes one candidate pressure family only. `eligible_until` remains in
the runtime for paired observation, but candidate pressure and unsupported-use
expiry must ignore it. Deletion is reserved for PD2.

## Frozen candidate: local load exchange

Let `Q` be the existing CPC1 participation impulse. Each ordinary pressure
epoch supplies one pressure quantum `Q` to every live ARROW.

```text
absorbed              = min(participation, Q)
participation         = participation - absorbed
local_pressure_load   = local_pressure_load + (Q - absorbed)
durable_loss          = floor(local_pressure_load / Q)
local_pressure_load   = local_pressure_load mod Q
resistance            = resistance - durable_loss
```

Time between pressure epochs relaxes participation using the existing CPC1
physical relaxation law. Pressure epochs must be evaluated in physical-time
order even when the host advances across several epochs in one call.

There is no comparison of participation against a protection threshold. A
full trace can absorb a full pressure quantum; a partial trace leaves a
proportional residual load. Residual sub-quantum pressure is future-causal
ARROW-local state, not discarded rounding.

`eligible_until` may still be written by traversal and cleared after its old
deadline for PD1 observation. It may not suppress ordinary pressure or cause
unsupported-use pressure.

## Frozen consequence conversion

The existing qualified local Modulatory arrival meets whatever participation
remains at the contact. PD1 records the existing graded support and converts
that same magnitude to an integer resistance gain bounded by the existing
`LOCAL_RETURN_STRENGTH = 3`:

```text
bounded_participation = min(participation, Q)
durable_gain = ceil(bounded_participation * 3 / Q)
```

Zero participation gives zero gain. Positive participation gives a magnitude-
dependent gain from one through three because durable resistance is integer.
Coupling is unchanged. Modulation does not consume participation; existing
PQLC continuation remains byte-identical. Repeated genuine consequences may
provide repeated durable evidence. Traversal, Drive, and pressure never add
durable resistance.

## Frozen matrix

Run every family at initial pressure phases `0..9`, under two fresh physical
identity roots, through both Reference and selected Production mechanics, plus
an exact same-mechanics replay.

### P0 never used

An untraversed resistance-one route receives pressure and disappears.

### P1 recently used

A resistance-one route traverses shortly before pressure. Its actual remaining
participation reduces the pressure effect, the route remains live, and durable
resistance does not increase.

### P2 same age, different magnitude

Three routes have the same most-recent traversal age but physically generated
low, medium, and high total participation magnitudes. At the same pressure
event, the newly admitted residual pressure load must be strictly ordered
`low > medium > high`. Equal Boolean protection fails this family.

No participation value may be seeded or directly mutated by the evaluator.

### P3 use without consequence

Repeated actual same-path traversal may keep a weak route provisionally live
while use continues. Durable resistance never increases. Once traversal stops,
participation relaxes and pressure eventually removes the route.

### P4 timely consequence

Actual traversal leaves enough participation to survive an intervening
pressure event. Qualified local Modulation then produces a positive durable
resistance gain computed from the remaining magnitude.

### P5 late maintained consequence

Across a delay longer than the old four-tick window, actual repeated same-path
participation maintains a real PQLC chain. A later qualified consequence may
close through the chain and durably strengthen still-participating contacts.

### P6 same delay, not maintained

Use the same wall-clock delay as P5 but provide no same-path maintenance.
Participation relaxes, pressure wins, and late Modulation cannot resurrect or
strengthen dead structure.

### P7 wrong-path activity

Provide equal or greater activity on a physically separate path while the
candidate is unsupported. Other-path activity may not absorb the candidate's
pressure, maintain its participation, or provide durable gain.

### P8 repeated pressure stress then stop

Alternate genuine same-path traversal and pressure over several epochs, then
stop traversal. The route may remain provisionally live during use, must show
zero traversal-only durable gain, and must disappear after use ceases.

### P9 time-partition invariance

Run the same physical history once with tickwise host advancement and once
with larger host jumps spanning multiple pressure epochs. Future-causal state,
ordered physical history, final body, work, and subsequent behavior must be
identical. Host call boundaries may not define pressure physics.

The unconditional inventory is `10 families * 10 phases * 2 roots = 200`
physical cases, `400` Reference/Production rows, and `800` same-mechanics
replay runs.

## Required observations

Serialize for every case:

- family, root, phase, mechanics, replay ordinal;
- every relevant candidate's liveness, resistance, coupling,
  `eligible_until`, participation, accumulated support, and pressure load;
- pressure event ticks and before/after participation, pressure load, and
  durable resistance;
- Drive and Modulatory deliveries, QLP traversals, resistance changes,
  proposals, deallocations, clock, pressure phase, pending activity, and
  PhysicalWork;
- canonical durable body, natural quiescence, and exact replay.

For P2, separately serialize the common most-recent traversal age and the
three pre-pressure participation magnitudes plus per-event pressure-load
deltas. For P5/P6, serialize the old eligibility flag beside the actual
participation to prove the old deadline is causally irrelevant.

Reference and Production must match exactly on the frozen future-relevant
physical observation. Causally inert sparse-versus-eager timestamp bookkeeping
remains excluded exactly as established before PD0; no new comparator repair
is allowed after observation.

## Static prohibitions

PD1 substrate logic may contain no pressure branch or multiplier based on:

- `eligible_until` or `LOCAL_WINDOW`;
- a Boolean `participation > 0` protection decision;
- task, family, delay, pressure phase, ARC, outcome, reward, credit, cause,
  route/path identity, history, depth, or expected answer;
- evaluator-provided participation, pressure load, or durable gain.

No new countdown, deadline, grace period, pressure skip, or eligibility alias
is permitted. No ARC input or curriculum fact enters the gate.

## Decision

- Any family, mechanics, replay, time-partition, quiescence, inventory, hash,
  or static-audit failure: PD1 stopped negative; freeze and stop.
- Complete evidence: PD1 development-positive only.

PD1 does not delete `eligible_until`, change PQLC topology, run ARC, modify
authority/oracle status, or update `arch.md`. PD2 remains separately blocked.
