# PD0 old pressure interaction characterization protocol v1

Status: frozen before any PD0 evaluator or Rust change.

Parent: PQLC1 development-positive result at
`ef5c80d4c64dbe306d1f91fbf45ec8acc8d5f9fc`, tagged
`pqlc1-depth-composition-result-v1`.

## Question

What physical behavior is currently supplied by the retained rectangular
eligibility-pressure exception when it coexists with CPC1 continuously
relaxing participation?

PD0 is characterization only. It selects no equation, constant, candidate, or
winner. It changes no runtime law and does not run ARC.

TC-DS0 already established the old credit cliff:

```text
return delay 0..4   -> rectangular eligibility admits return
return delay 5+     -> rectangular eligibility rejects return
```

PD0 does not repeat that claim. It measures the separate pressure interaction
introduced later:

```text
ordinary pressure epoch
    covered by eligible_until -> decrement suppressed

eligibility expires unsupported
    -> separate unsupported-use decrement
```

It also serializes CPC1 participation and local plastic support beside that
old bookkeeping so PD1 can compare candidates against an exact independent
baseline.

## Frozen substrate

Use the exact PQLC1 ancestor with feature `pqlc0` (therefore `cpc1`). The
following remain byte-identical:

- PQLC0 trigger/effect law;
- CPC1 participation impulse and relaxation arithmetic;
- current `eligible_until = traversal_tick + 4` bookkeeping;
- ordinary pressure period and eligibility-covered pressure suppression;
- unsupported-use pressure;
- Reference and Production mechanics.

Under CPC1, Modulation currently writes graded local `plastic_support`; it does
not yet change durable resistance. PD0 must expose, not repair, this boundary.

## Variables

```text
initial resistance       1, 2, 4
initial pressure phase   0..9
return/activity delay    0,1,2,3,4,5,8,12
renewal delay            1,2,3,4,5,8,12
observation horizon      initial tick + 60
```

Every CELL is separated beyond the local proposal radius. No fixture-created
proposal is expected.

## Frozen families

### Dormant

The weak candidate never traverses. Advance through the full horizon and
record every ordinary pressure epoch, resistance transition, and physical
deallocation.

`10 phases * 3 resistances = 30` physical cases.

### Traversed, no consequence

Traverse the candidate once, then provide no Modulation or other activity.
Record participation relaxation, rectangular eligibility, protected and
unprotected pressure epochs, unsupported expiry, resistance, and death.

`30` physical cases.

### Timed consequence

Traverse once and deliver ordinary Modulation at the same contact after each
frozen delay. Immediately before delivery, serialize participation magnitude,
eligibility state, liveness, and resistance. After delivery, serialize local
plastic support and the remaining durable trajectory through the horizon.

`10 * 3 * 8 = 240` physical cases.

### Unrelated activity

Traverse once, then deliver an equal ordinary Drive episode on a physically
separate path after each frozen delay. It must not be interpreted as candidate
Modulation or same-path traversal. Continue through the horizon.

`240` physical cases.

### Same-path renewal without Modulation

Traverse once and attempt the same physical traversal again after each frozen
renewal delay. If the structure remains live, the second actual traversal may
renew participation and rectangular eligibility. No Modulation occurs. Record
whether repeated use changes durable resistance and whether the route survives
after activity stops.

`10 * 3 * 7 = 210` physical cases.

The unconditional inventory is therefore `750` physical cases and `1500`
Reference/Production mechanics rows, before `3000` exact same-mechanics replay
runs are counted.

## Required serialization

For each case serialize:

- initial phase, resistance, family, and delay;
- every-tick candidate trajectory:
  `tick/live/resistance/coupling/eligible/participation/support`;
- emitted eligibility deadlines;
- pressure-epoch ticks;
- eligible pressure epochs and whether resistance remained unchanged;
- ineligible pressure epochs and whether pressure reduced/deallocated;
- participation and eligibility immediately before the delayed event;
- support and resistance immediately after it;
- final durable candidate state;
- Drive and Modulatory deliveries, plastic updates, proposals, deallocations,
  and PhysicalWork;
- clock, pressure phase, canonical durable body, natural quiescence, and exact
  replay.

Reference and Production must match exactly on the complete causal observation
and future-relevant state. Causally inert sparse-versus-eager CELL timestamp
bookkeeping remains outside cross-mechanics comparison exactly as established
by TC-DS0 v2.

## Characterization summaries

The result must report without ranking:

- eligible pressure epochs protected / observed;
- ineligible pressure decrements or deaths / observed;
- support-positive cases by delayed-event age;
- durable resistance gains caused by traversal alone;
- durable resistance gains after CPC1 Modulation;
- final liveness by family and initial resistance;
- same-path renewal attempts that occurred before versus after deallocation.

No summary value is a candidate-selection score.

## Static prohibitions

PD0 may not change the core, constants, checkpoints, pressure, participation,
PQLC, or mechanics. The evaluator may observe public state and reconstruct the
old eligibility flag from emitted physical eligibility plus Modulatory
consumption; it may not mutate hidden state.

No ARC input, curriculum information, task identity, expected outcome, or
candidate PD1 equation may enter PD0.

## Decision

- Any mechanics, replay, quiescence, hash, inventory, or static-audit failure:
  PD0 stopped negative; freeze and stop.
- Complete exact evidence: PD0 characterization positive, regardless of
  whether the old interaction looks desirable.

PD1 remains separately blocked until PD0 is frozen and reviewed. No pressure
candidate, eligibility deletion, ARC A2 replay, ARC A3-A5, authority, oracle,
or `arch.md` change occurs in PD0.
