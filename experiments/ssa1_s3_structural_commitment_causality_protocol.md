# SSA1-S3 structural-commitment causality protocol

Status: **development preregistration; no definitive execution authorized**.

Lineage:

```text
Frozen Organism v1
  -> SSA1-S Classification E
  -> SSA1-S2 Classification B
       P8 perfectly classifies the final basin, but only after structural change
  -> SSA1-S3 structural-commitment causality
```

SSA1-S3 does not amend, rescue, or rerun any predecessor. Frozen Organism v1,
SSA1, SSA1-C1/C2/R/S/S2, and their evidence remain immutable. SSA2 remains
blocked.

## Frozen inputs

| input | immutable identifier |
|---|---|
| S2 parent commit | `71d42fa03b2b715736d01a181faba6f842a74d4c` |
| S2 final tag | `ssa1-s2-application-history-predictor-development-classification-b` |
| Frozen substrate SHA-256 | `6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263` |
| M5 plasticity-allocation SHA-256 | `e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7` |
| M6 consequence/credit SHA-256 | `11b4229122b3e0788ca30c55579b91ffe07461de9a138860690134565fcf2ed6` |
| S2 evaluator SHA-256 | `5e9f2055a4ec036f8adbe7c89de7028d2772826b2f5afea4bc97f99ca19d5c57` |
| S2 GATE CSV SHA-256 | `164ea561cff5ba910dfb5cc9c2b781de05feb83761afc7582b9ce16cb74ad6cd` |

The inserted evaluator adapter may invoke only frozen `begin_event` and
`local_encounter` operations on physically present encounters. It may inspect
state. It may not edit M5/M6 values, evidence, application counts, proposal
records, lifetime resistance, basin labels, or thresholds directly.

## Research question

> Is S2's P8 state a causal physical commitment boundary—formed by alternative
> threshold crossing and incumbent deallocation—or only a perfect late readout
> of some hidden accumulated evidence score?

## Fixed developmental world

Use byte-identical S2 maturation (`H=8`), opportunity construction, consequence
construction, 90-event periodic schedules, and `18,000` changed-world episodes.
The primary frozen anchor is the first lexicographic S2 GATE alternative-basin
row with both structural events:

```text
seed/base identity       2,030,000,000
B:A ratio                1:2
stride                   7
offset                   1
incumbent side/route     0 / 0
alternative route        1
S2 B threshold episode   1,529
S2 A deallocation        12,509
S2 final structure       [1,4] ALTERNATIVE
```

The event numbers are frozen-parent observations, not S3 evidence. Each S3
cell first reproduces the unmodified reference and locates its own two physical
transitions by live-supporter state. A cell is unresolved if the reference does
not reproduce both transitions and the final alternative basin.

## Causal branching

Clone the complete frozen session immediately before each observed transition.
Paired arms therefore have byte-identical developmental history up to the
intervention.

### T0 — reference

Run the unchanged schedule. The alternative reaches four live supporters and
the incumbent later falls below four. The final basin must be `ALTERNATIVE`.

### T1 — prevent alternative threshold crossing

At the exact event where the reference alternative changes from fewer than four
to four live supporters, make the alternative encounters physically absent for
that event only. This is implemented by not delivering those local encounters
to the frozen path. Do not change their records or M5/M6 state. Resume the
unchanged schedule immediately afterward.

Preregistered causal prediction: the alternative does not cross at that event
and the final basin is `INCUMBENT_LOCK`.

### T2 — prevent incumbent deallocation

At the exact event where the reference incumbent would fall from at least four
to fewer than four live supporters, deliver one ordinary recurrence of each
currently live incumbent encounter immediately before ordinary event pressure.
The recurrence uses frozen `local_encounter`; no resistance or record is set by
the evaluator. Resume the unchanged schedule immediately afterward.

Preregistered causal prediction: the alternative crosses, the incumbent does
not deallocate at that event, and the final basin is `MIXED`.

### T3 — post-commitment timing controls

Apply the identical T1 absence and T2 recurrence operations one event after the
corresponding structural transition in independent reference clones. They must
not undo the transition that has already occurred and must not change the
reference final basin.

The evaluator may name these arms and measure transitions. Those names and
measurements never enter the organism.

## What counts as a physical intervention

Allowed:

- withhold the ordinary local encounters of one route for one event;
- deliver an ordinary recurrence of already-existing local encounters;
- clone and replay complete physical/learner state;
- measure live supporter count before and after an event.

Forbidden:

- setting/deleting a proposal, value, evidence record, or lifetime scalar;
- changing M5 score, M6 evidence, application order, consequence variant, or
  future schedule;
- supplying `ALTERNATIVE`, `MIXED`, `LOCK`, `COMMIT`, `THRESHOLD`, or
  `DEALLOCATE` to the learner;
- evaluator-selected winner, reward, semantic credit, RNG, replay learning, or
  a new choice/reopening mechanism.

## Required controls

1. Reference execution is duplicate-exact and reproduces frozen S2 transitions.
2. Prefix clones are byte/state exact immediately before intervention.
3. Each intervention changes only delivered physical encounter activity.
4. Scheduled opportunities and consequences after intervention remain exact.
5. A physically absent/stale route cannot execute in the intervention event.
6. Post-commitment activity remains physically observable but is too late to
   reverse the already-completed structural event.
7. Incumbent-side mirror, route-handle permutation, occurrence identity,
   allocation/layout reversal, and fresh identities preserve relations.
8. Removing evaluator trace collection leaves final states exact.
9. Frozen-parent source and result hashes remain exact.
10. No predecessor artifact or definitive surface is written or executed.

## Development stages

### PROBE

Use the frozen primary anchor and its A/B mirror. Establish exact prefix cloning,
that T1 reaches the threshold-forming event, and that T2 recurrence reaches the
pressure/deallocation event. Freeze the result regardless of classification.

### MICRO

Use two fresh identities with route and incumbent-side mirrors. Execute T0-T3,
the full tail, and all causal controls. Freeze any failure; do not increase the
intervention duration or strength after observing it.

### GATE

Use six fresh identity/layout cells and three frozen S2 alternative-producing
schedule descriptors (`1:2 / 7 / 1`, `1:2 / 13 / 43`, and `1:2 / 17 / 1`).
Every cell must satisfy the same conjunctive causal signature. No definitive
execution is authorized.

## Development classifications

- **A — structural commitment causal:** T1 prevents the alternative transition
  and ends incumbent-locked; T2 preserves the incumbent and ends mixed; the
  corresponding post-transition operations are inert; all controls pass.
- **B — threshold causal only:** T1 has the predicted causal effect but T2 does
  not preserve a mixed basin.
- **C — deallocation causal only:** T2 has the predicted causal effect but T1
  does not preserve incumbent lock.
- **D — P8 readout, not established causal boundary:** neither node-level
  intervention produces its predicted basin despite reaching the exact event.
- **E — scientific ambiguity:** the frozen substrate cannot isolate the physical
  transition without also changing M5/M6 evidence or future experience.

No outcome changes SSA1's frozen Classification C or modifies Frozen Organism
v1.

## Stopping rule

Freeze and return after the first complete GATE classification, or earlier if a
scientific ambiguity prevents a lawful intervention. Preserve every negative.
Do not tune intervention strength, duration, schedule, or target episodes after
observing S3 evidence. SSA2 remains blocked pending interpretation.
