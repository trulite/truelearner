# SSA1-R rich changing world protocol

Status: **development preregistration; no definitive execution authorized**.

Lineage:

```text
Frozen Organism v1
  -> SSA1 Classification C
  -> SSA1-C1 Classification C
  -> SSA1-C2 Classification A + B-only credit absorber
       064b169
  -> SSA1-R rich changing world
```

SSA1-R is a separately named functional-sufficiency successor. It does not
promote or amend SSA1, SSA1-C1, or SSA1-C2. Frozen Organism v1 remains
unchanged, SSA2 remains blocked, and architecture reopening is not authorized.

## Frozen inputs

| input | SHA-256 |
|---|---|
| Frozen Organism v1 substrate | `6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263` |
| substrate source | `e49578f050f75fe0be181930d6231815abdbdc382b1b5b8c690cb19a637b68d3` |
| M5 plasticity-allocation source | `e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7` |
| M6 consequence/credit source | `11b4229122b3e0788ca30c55579b91ffe07461de9a138860690134565fcf2ed6` |
| SSA1 evaluator | `dc157e0bd238992d6475e5dc9767c6f7711a1bb5b7759ebdb7991573aea5199b` |
| SSA1-C2 evaluator | `c7a785763d9283bd213a951a7c0fd378d8d9b63a3e3717cf51d250ff25ce6a8d` |
| SSA1-C2 GATE | `21e9709ff3f1a98f4212b82811719ed6f1db3ec7f8c35b0ccd9ba4df45d5aede` |
| SSA1-C2 handoff | `d20ccdb3e5c7646571840a72aecdb8ee17d48ae8e74e19f59bd2af0f2a053602` |

The frozen C2 evaluator-only audit surface may be reused to observe M5/M6
state. It may not edit it.

## Research question

> Can ordinary rich experience supply enough ongoing physical and consequence
> contrast for Frozen Organism v1 to autonomously maintain and reorganize an
> appropriate executable affordance landscape, without a forced alternative,
> paired diagnostic intervention, or exploration curriculum?

This tests functional sufficiency in an environment, not spontaneous recovery
under the original SSA1 World C protocol.

## Fixed rich-world physics

Every world contains two genuine executable CELL/ARROW/SPIKE continuations at
two physical locations. Each route reaches a distinct downstream physical
effect. Route names exist only in the evaluator.

### Exogenous transient field

An environment-local deterministic field runs continuously from episode zero.
It is fixed before the learner starts and never reads:

- learned support, admission, value, or resistance;
- which route is currently dominant or suppressed;
- consequence regime or desired winner;
- prior success or failure.

The field's physical side follows a balanced Thue-Morse schedule derived from
the environmental clock. Its burst amplitude follows an independent fixed
eight-step cycle:

```text
0, 1, 2, 3, 1, 3, 2, 0
```

Amplitude means the number of ordinary early local spikes reaching that
physical side before closure. The two burst-3 positions provide naturally
recurring histories capable of completing a one-support affordance. Smaller
bursts remain subthreshold for such an affordance.

The side schedule, amplitude cycle, and consequence clock use different
periods/phases. Their cross-product must be balanced within every measured
regime. The evaluator may verify balance after the fact; it may not change the
field to obtain an execution.

The field is attached to physical location. Under route-identity and layout
permutations, physical relations are preserved and route labels change. Exact
complete-state replay remains deterministic.

### Consequence regimes

Downstream consequences are ordinary equal-magnitude physical activity. A
stable side repeatedly produces one normalized consequence shape. A variable
side produces a fixed four-shape sequence from an environmental clock that is
independent of the transient field.

No semantic reward, correctness, usefulness, novelty, diversity, regime label,
or comparison is supplied. M6 sees only the physical consequence histories of
routes that actually executed.

## Preregistered worlds

### R0 — stationary winner

One physical side remains stable and the other variable for 10,000 episodes.
The rich transient field is active throughout.

Expected functional signature:

- both sides physically execute early enough to generate comparison;
- the stable side becomes M6-eligible and M5-favored;
- final live support is the stable side at 4 and the variable side below 4;
- stationary continuation does not create false reopening;
- A/B mirror and physical-layout permutation preserve the relation.

### R1 — multi-useful stationary world

Both physical sides produce stable equal-strength consequence histories for
10,000 episodes. Neither side is privileged.

Expected functional signature:

- both routes remain independently executable with four live supporters;
- the field realizes both routes across its ordinary histories;
- M6 abstains rather than inventing a differential;
- handle order, route index, and layout do not break the equivalence.

### R2 — naturally changing and returning world

From a blank organism, an exogenous regime oscillator runs three fixed phases:

```text
phase 1: left stable,  right variable
phase 2: left variable, right stable
phase 3: left stable,  right variable
```

No reset, forced route, block, paired trial, or adaptive intervention occurs at
phase boundaries. The same transient field and organism run continuously.

Run three separately frozen dwell-time worlds:

```text
512 episodes per phase
4,096 episodes per phase
10,000 episodes per phase
```

The dwell time is a physical property of the environment, not a curriculum
chosen in response to the learner. All three are preregistered before evidence.

For every phase record:

- physical executions and returned consequences by side;
- M6 evidence shapes, support, margin, eligibility, and abstentions;
- M5 support, rejection, score, resistance, and applications;
- admissions, live supporters, proposal resistance, and exploration admissions;
- independent field-free execution at the phase boundary.

Full autonomous tracking at a dwell time requires:

```text
phase 1 -> [4, <4]
phase 2 -> [<4, 4]
phase 3 -> [4, <4]
```

with independent execution matching the currently stable side.

Partial tracking is reported if the correct side remains viable but physical
dominance does not reverse before a phase ends.

## Causal controls

At minimum:

1. **No rich field:** use the identical changing consequence oscillator but no
   environmental bursts. This tests whether naturally recurring alternative
   physical histories are required.
2. **Post-closure field:** move the same bursts after closure. They must be
   physically visible but inert for the resolved trajectory.
3. **Stationary consequence control:** keep consequences stationary while the
   field continues. The landscape must not toggle merely because the field
   moves.
4. **Clock-phase permutation:** change the initial phase relation among field,
   amplitude, and regime clocks without changing any law.
5. **Fresh identities and A/B mirror:** transfer across occurrence, route,
   handle, cell, arrow, and physical-side identities.
6. **Allocation/layout permutation:** preserve physical relations under
   different insertion and storage order.
7. **Stale/blocked route:** a physically unavailable continuation cannot win
   because the field visits its side.
8. **Exact duplicate:** complete-state duplicate replay is exact.
9. **Balance audit:** each physical side receives matched counts of every burst
   amplitude within the measured regime, up to a preregistered difference of at
   most one when a finite prefix ends mid-cycle.

## Anti-pairing audit

The source and runtime must establish conjunctively:

- field side/amplitude are functions only of environmental clock and fixed
  physical layout;
- consequence regime is a separate function of environmental clock;
- the environment never queries the organism before scheduling the episode;
- there is exactly one ordinary episode and at most one consequence per
  realized route;
- no second forced comparison episode is appended;
- no route is blocked or enabled based on its learned status;
- no outcome causes the next physical history to be changed.

Failure of this audit invalidates any rich-world claim.

## Development stages

### PROBE

One fresh seed and mirror. Run R0, R1, and R2 at dwell 4,096 with minimal
controls. Establish that naturally scheduled burst-3 histories sometimes let a
suppressed one-support route execute, and that its real consequence returns
through frozen M6/M5.

### MICRO

Two fresh seeds and mirrors. Run R0, R1, all three R2 dwell times, no-field,
post-closure, stationary, clock-phase, stale-route, balance, and exact-replay
controls. Freeze a functional-sufficiency classification.

### GATE

Six fresh seeds with alternating identity/layout, the complete MICRO matrix,
full frozen-hash and anti-pairing audits, and conjunctive controls.

No definitive execution is authorized by development readiness.

## Development classifications

- **A — autonomous rich-world landscape control:** R0 and R1 pass, and R2
  tracks A -> B -> A at one or more preregistered dwell times in every fresh
  cell without forced or paired intervention.
- **B — partial rich-world adaptation:** R0 and R1 pass and changing-world
  evidence moves the landscape in the correct direction, but no dwell-time R2
  world completes both reversals in every cell.
- **C — contrast available but functionally insufficient:** both sides execute
  and M6 observes changing comparisons, but mature M5 allocation does not track
  the natural regimes.
- **D — natural counterexperience unavailable:** the fixed rich field does not
  keep/reopen suppressed physical execution, so comparative evidence cannot
  enter.
- **E — scientific ambiguity:** the fixed world cannot separate physical
  opportunity from consequence contrast without adding machinery.

Classification A would establish functional SSA1 sufficiency in this tested
class of rich environments. It would not retroactively change SSA1's original
Classification C.

## Frozen count-capacity boundary

Every world remains below the frozen `u16` per-shape evidence capacity. No
single route can execute more than 30,000 times in the longest three-phase
world. No counter is widened, split, wrapped, or reset.

## Hard exclusions

Forbidden:

- changes to Frozen Organism v1, M5, M6, SSA0.3, SSA1, C1, or C2;
- forced alternative execution or adaptive background support;
- paired diagnostic trials or evaluator-selected comparison;
- an exploration curriculum chosen from learner state;
- evaluator-selected winner, credit, plastic site, proposal, or consequence;
- counterfactual evidence for an unexecuted route;
- RNG, noise, probability, sampling, novelty, diversity, or curiosity state;
- new context, regime, phase, or utility representation in the organism;
- definitive evidence, SSA2, scaling, or architecture reopening.

## Stopping rule

Freeze and return when SSA1-R reaches a development classification across the
preregistered matrix, or stop earlier for genuine scientific ambiguity. Do not
tune the field, dwell times, or clocks after observing a failure. Preserve all
negative worlds independently.
