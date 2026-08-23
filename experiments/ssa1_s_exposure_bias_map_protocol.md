# SSA1-S selection-induced exposure bias map protocol

Status: **development preregistration; no definitive execution authorized**.

Lineage:

```text
Frozen Organism v1
  -> SSA1 Classification C
  -> SSA1-C1 Classification C
  -> SSA1-C2 Classification A under sufficient paired changed-world contrast
  -> SSA1-R Classification C under fixed natural rich-world contrast
       afb8ea8
  -> SSA1-S exposure bias map
```

SSA1-S is characterization, not another attempt to make SSA1 pass. It cannot
amend any prior classification, modify Frozen Organism v1, advance SSA2, or
authorize architecture reopening.

## Frozen inputs

| input | SHA-256 |
|---|---|
| Frozen substrate | `6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263` |
| organism module | `e49578f050f75fe0be181930d6231815abdbdc382b1b5b8c690cb19a637b68d3` |
| M5 plasticity allocation | `e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7` |
| M6 consequence/credit | `11b4229122b3e0788ca30c55579b91ffe07461de9a138860690134565fcf2ed6` |
| SSA1 evaluator | `dc157e0bd238992d6475e5dc9767c6f7711a1bb5b7759ebdb7991573aea5199b` |
| SSA1-C2 evaluator | `c7a785763d9283bd213a951a7c0fd378d8d9b63a3e3717cf51d250ff25ce6a8d` |
| SSA1-R evaluator | `cad88a1bbd02d8a6154393c6992f57cb4650f7342e9fb59691db79da0d28b734` |
| SSA1-R GATE | `8d5ec14ebb48af250486c30b3721e1b66a9474303eeb10a50bfb7ba1b786c003` |
| SSA1-R handoff | `3b51b139f1cb815ed9a80bc396011065ba2231cddf4d36f12cf093240e157ca8` |

The existing C2 audit surface may observe frozen M5/M6 state. It may not edit
that state.

## Research question

> How does independently generated environmental opportunity transfer into
> actual route exposure, M6 evidence, M5 allocation, and the final executable
> landscape as a function of prior learning maturity?

The target is a phase diagram, not a preferred pass/fail outcome.

## Fixed physical world

Every cell contains two genuine executable CELL/ARROW/SPIKE continuations on
two physical sides. Evaluator names `incumbent` and `alternative` do not enter
the organism.

### Initial-history axis

From blank starts, the environment supplies one fixed amount of incumbent-only
prehistory:

```text
0, 2, 8, 32, 128 episodes
```

Each prehistory episode contains an ordinary three-spike early local event at
the incumbent physical side and one stable physical consequence if that route
executes. The schedule is fixed by the matrix cell, never selected from learner
state. It exists only to establish the preregistered maturity axis already
motivated by C2.

Record the complete M5/M6 audit and executable landscape at the boundary.

### Changed-world consequence law

After the maturity boundary, the world changes once and remains changed:

```text
incumbent route  -> fixed four-shape variable consequence sequence
alternative route -> one stable consequence shape
```

All consequences have equal physical magnitude. There is no reward, utility,
correctness, desired route, regime metadata, or counterfactual evidence.

### Environmental-opportunity axis

Cross every maturity with these alternative:incumbent opportunity ratios:

```text
1:8
1:4
1:2
1:1
2:1
4:1
8:1
```

An opportunity is an ordinary three-spike early local event at one physical
side, arriving at ticks `0, 2, 4` before closure. It perturbs substrate physics;
it does not select a route in the harness. A stale, absent, or physically
blocked route still cannot execute.

For a ratio `B:A = b:a`, the side schedule is the fixed balanced mechanical
word:

```text
B at episode t iff
floor((t + 1) * b / (a + b)) > floor(t * b / (a + b));
otherwise A.
```

The word is fixed before the organism starts. It depends only on environmental
clock, `a`, `b`, and a preregistered phase offset. It never reads support,
winner, execution, evidence, consequence, admission, or outcome.

Run exactly `18,000` changed-world episodes. This is divisible by every ratio
period. Record checkpoints at:

```text
0, 90, 900, 4,500, 9,000, 18,000
```

No trajectory may exceed frozen per-shape evidence capacity.

## Measurements

At every checkpoint record, in physical-side and route coordinates:

- scheduled environmental opportunities;
- actual executions and unresolved episodes;
- returned physical consequences;
- opportunity-to-execution transfer ratio;
- M6 evidence shapes, observations, support, margin, eligibility, and
  abstentions;
- M5 support, rejection, score, value resistance, applications, and
  exploration admissions;
- live proposal/supporter counts and proposal resistance;
- independent field-free execution at the checkpoint;
- exact duplicate physical fingerprints.

Classify the final landscape as:

```text
INCUMBENT_LOCK  incumbent >= 4, alternative < 4
MIXED           incumbent >= 4, alternative >= 4
ALTERNATIVE     incumbent < 4, alternative >= 4
SUBTHRESHOLD    both < 4
```

For each maturity, report the smallest environmental B:A ratio, if any, that
produces `ALTERNATIVE` by 18,000 episodes. Do not interpolate an unmeasured
threshold.

## Required causal controls

1. **Schedule accounting:** observed opportunity counts equal the exact
   preregistered ratio at every complete period and at 18,000.
2. **Anti-adaptation:** source and runtime prove the opportunity is scheduled
   before querying the organism and no outcome changes the next schedule.
3. **Opportunity transfer:** a live one-support route plus three early physical
   spikes can execute; an absent/stale route cannot.
4. **Post-closure opportunity:** the same three spikes after closure are
   physically visible but cannot change the already realized trajectory.
5. **Equal-consequence control:** at `1:1`, both sides return the same stable
   consequence. Record whether exposure symmetry preserves both routes or
   spontaneously breaks; do not require a preferred result.
6. **Schedule-phase control:** rotate the fixed opportunity word without
   changing its ratio; the transfer curve must not depend on the first handle.
7. **Fresh identities and A/B mirror:** alternate which physical side is the
   incumbent and permute occurrence/route/handle identities.
8. **Allocation/layout permutation:** reverse insertion/storage order while
   preserving physical relations.
9. **Exact duplicate:** identical complete physical state and opportunity
   replay exactly.
10. **Frozen-parent hashes:** all listed inputs remain exact.

## Anti-cheat exclusions

Forbidden:

- `choose(A,B)`, forced route execution, learner-dependent blocking, or an
  appended paired comparison trial;
- schedule changes based on support, winner, evidence, failure, or outcome;
- semantic reward, correctness, utility, novelty, curiosity, diversity, or
  exploration state;
- counterfactual evidence for a route that did not execute;
- inverse-propensity correction, evidence normalization, importance weighting,
  replay buffers, or any other exposure-bias correction;
- RNG, softmax, temperature, stored probabilities, or harness sampling;
- changes to Frozen Organism v1, M5, M6, SSA0.3, SSA1, C1, C2, or R;
- definitive evidence, SSA2, scaling, or architecture reopening.

## Development stages

### PROBE

One fresh seed plus mirror. Use maturities `0, 32, 128` and opportunity ratios
`1:4, 1:1, 4:1`. Establish that the fixed ecology reaches real competition and
that the full opportunity -> execution -> M6 -> M5 audit chain is measurable.

### MICRO

Two fresh seeds plus mirrors. Run the complete `5 x 7` phase map, all
checkpoints, equal-consequence, schedule-phase, stale, post-closure, accounting,
and duplicate controls. Freeze the first exposure-bias classification.

### GATE

Six fresh identity/layout cells. Transfer the complete MICRO map and all
conjunctive controls. No definitive run is authorized by GATE readiness.

## Development classifications

- **A — coherent selection-induced exposure phase map:** environmental
  opportunity changes actual exposure and evidence monotonically in the tested
  ordering, maturity shifts the measured allocation boundary monotonically or
  leaves it unchanged, and all controls pass.
- **B — non-monotonic but resolved exposure map:** the complete transfer curve
  is valid and controlled, but one or more internal transitions are
  non-monotonic or maturity-dependent in a way directly resolved by frozen
  state inspection.
- **C — exposure-insensitive allocation:** opportunity and execution exposure
  change, but no tested ratio changes the learned allocation boundary.
- **D — physical opportunity does not transfer:** the fixed external events do
  not generate the intended range of actual exposure, so the credit/allocation
  curve cannot be identified.
- **E — scientific ambiguity:** more than one uncontrolled physical variable
  prevents attribution without adding machinery.

Classification A or B characterizes a substrate-development law; neither makes
SSA1 positive. Classification C is also scientifically complete if physical
opportunity and evidence were successfully varied.

## Stopping rule

Freeze and return after the preregistered GATE classification, or stop earlier
only for genuine scientific ambiguity. Preserve every failed stage unchanged.
Mechanical audit/reporting defects may be corrected only after freezing the
failed artifact, without changing the organism, opportunity law, ratios,
maturities, consequences, budgets, or claims.
