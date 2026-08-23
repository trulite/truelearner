# SSA1-S2 application-history predictor protocol

Status: **development preregistration; no definitive execution authorized**.

Lineage:

```text
Frozen Organism v1
  -> SSA1-S Classification E
       ratio x maturity map invalidated by temporal phase
       e937329
  -> SSA1-S2 application-history predictor
```

SSA1-S2 does not rerun, rescue, amend, or reinterpret SSA1-S. It treats the
temporal-order dependence as the target phenomenon. Frozen Organism v1 remains
unchanged and SSA2 remains blocked.

## Frozen inputs

| input | SHA-256 |
|---|---|
| Frozen substrate | `6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263` |
| organism module | `e49578f050f75fe0be181930d6231815abdbdc382b1b5b8c690cb19a637b68d3` |
| M5 plasticity allocation | `e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7` |
| M6 consequence/credit | `11b4229122b3e0788ca30c55579b91ffe07461de9a138860690134565fcf2ed6` |
| SSA1-S evaluator | `9bbfed24dc0e70b5c5db65c214385124db928ff4cf6ae84229ef4120e1d23b22` |
| SSA1-S MICRO | `722902179f74298ae17b86f403b5c8988319fd42f4b0802e87f2843aec9ab989` |
| SSA1-S handoff | `e5ff49970ef4344044ce83a01baca712b49910259f235f9b686345a629af5207` |

The frozen C2 audit surface may be sampled before and after ordinary episodes
to observe M5/M6 state. It may not edit that state.

## Research question

> What is the smallest preregistered evaluator-side summary of the actual M6
> application history that predicts which M5/affordance basin is entered under
> different schedules containing the same multiset of physical experiences?

No summary or prediction is exposed to the organism.

## Fixed developmental world

The main map holds maturity at the SSA1-S transition value `H=8`. Initial
prehistory, physical opportunity, changed-world consequences, route execution,
and M5/M6 learning are byte-identical to SSA1-S.

Use the four SSA1-S ratios whose H=8 outcome changed or occupied different
basins:

```text
B:A = 1:2, 1:1, 2:1, 4:1
```

Every schedule runs exactly `18,000` changed-world episodes. Within a ratio,
all schedules contain exactly the same number of incumbent and alternative
opportunities, executions when physically available, and possible consequence
returns. Only temporal order differs.

Sentinel controls at `H=2` and `H=32` use the same schedule family subset to
verify transfer outside the transition band; they are not used to select a
predictor.

## Deterministic permutation family

Construct a 90-episode macro-word. For ratio `B:A=b:a`, let:

```text
B_count = 90 * b / (a + b)

B at macro position t iff
((stride * t + offset) mod 90) < B_count
```

Repeat the macro-word 200 times. No RNG or learner state enters schedule
construction.

Use these preregistered coprime strides:

```text
1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43
```

and offsets:

```text
0, 1, 17, 43
```

This creates 48 schedules per ratio and 192 transition-band trajectories per
identity cell. Each 90-event macro contains the exact same multiset for a given
ratio while local run length, alternation, and first event vary.

The descriptor split is fixed before evidence:

```text
DISCOVERY  (stride_index + offset_index) even
HELD_OUT   (stride_index + offset_index) odd
```

Each ratio contributes 24 discovery and 24 held-out schedules.

PROBE may use only strides `1, 7, 11` and offsets `0, 1`. MICRO and GATE use
the complete family.

## Evaluator trace

For every ordinary episode, evaluator-side snapshots may record:

- scheduled and realized physical role;
- consequence shape returned;
- M6 observations, support, margin, eligibility, abstention count, and
  application count before and after the return;
- M5 support, rejection, score, and value resistance before and after;
- live supporters before offer, after offer, and after consequence return;
- first alternative threshold crossing and first incumbent deallocation;
- exact physical execution fingerprints.

An **effective directional application** occurs only when frozen M6's
application count increases and the role-relative M5 score gap changes.

```text
direction = sign(
  delta(alternative M5 score - incumbent M5 score)
)
```

Direction is `+1` alternative, `-1` incumbent, or `0` neutral. If the audit
cannot uniquely attribute an application direction, the trajectory is marked
unresolved; do not infer a semantic direction from the world label.

## Preregistered candidate summaries

All predictors map a signed statistic mechanically:

```text
negative -> INCUMBENT_LOCK
zero     -> MIXED
positive -> ALTERNATIVE
```

`SUBTHRESHOLD` is always an incorrect prediction for this library and must be
reported separately if observed.

Evaluate this ordered nested library:

| ID | evaluator-side summary |
|---|---|
| P0 | opportunity ratio only; discovery-set majority basin per ratio |
| P1 | direction of first effective M6 application |
| P2 | signed balance of first 4 effective applications |
| P3 | signed balance of first 8 effective applications |
| P4 | signed balance of first 16 effective applications |
| P5 | sign of role-relative M5 score gap immediately before the first opposing application |
| P6 | direction with the longest contiguous run among the first 90 effective applications; ties predict MIXED |
| P7 | sign of role-relative M5 score gap after episode 90 |
| P8 | structural commitment: alternative crosses threshold and incumbent deallocates -> ALTERNATIVE; both live -> MIXED; otherwise INCUMBENT_LOCK |
| P9 | tuple `(P3, P5, P8)` using discovery-set majority for the exact tuple |

P0 and P9 may learn only their finite categorical lookup from discovery
schedules. P1-P8 are fully specified rules and receive no fitted parameters.

For every predictor report on discovery and held-out schedules:

- total accuracy;
- per-basin accuracy;
- coverage when a required event never occurs;
- identity/mirror transfer;
- earliest episode at which its statistic becomes available.

The selected predictor is the lowest-numbered P1-P9 satisfying conjunctively:

```text
discovery accuracy >= 95%
held-out accuracy  >= 95%
held-out coverage  >= 90%
every physical mirror >= 90% accuracy
```

P0 is a required baseline and cannot establish a mechanistic predictor.

## Controls

1. Every schedule has exact ratio counts per macro and at 18,000.
2. Matched schedules differ only in order; no adaptive opportunity, block, or
   appended paired trial is permitted.
3. Opportunity is scheduled before querying the organism.
4. Exact complete-state duplicate execution remains exact.
5. Fresh occurrence/route identities, incumbent-side mirror, handle
   permutation, and allocation-layout reversal preserve physical relations.
6. A stale route cannot execute because its scheduled opportunity arrives.
7. Post-closure opportunities remain physically visible but inert.
8. The trace is observational: removing trace collection produces the same
   final landscape and frozen-state fingerprint.
9. H=2 and H=32 sentinel schedules retain their previously established broad
   plastic/mature relation.
10. All frozen-parent hashes remain exact.

## Development stages

### PROBE

One fresh identity plus mirror; six schedules per ratio from the restricted
family. Verify trace attribution, multiple basins at H=8, and candidate-summary
accounting. Freeze any attribution failure.

### MICRO

Two fresh identities plus mirrors; complete 192-schedule H=8 family, H=2/H=32
sentinels, discovery/held-out split, predictor library, and all controls. Freeze
the first predictor classification.

### GATE

Six fresh identity/layout cells. Recompute the same fixed predictor library and
require the same lowest qualifying predictor and accuracy class. No definitive
run is authorized.

## Development classifications

- **A — early low-dimensional application law:** P1-P6 is the same lowest
  qualifying predictor in every MICRO/GATE cell. Basin selection is predictable
  from a small early summary of M6 application order.
- **B — commitment-state law:** no P1-P6 qualifies, but P7 or P8 qualifies.
  Order compresses only after M5/physical commitment is already substantially
  formed.
- **C — composite history law:** only P9 qualifies. The tested history can be
  compressed, but not to one primitive statistic.
- **D — sequence-complex within tested library:** no P1-P9 qualifies despite
  complete trace attribution and controls.
- **E — scientific ambiguity:** application direction cannot be uniquely
  observed or a control prevents causal attribution.

No classification makes SSA1 positive or authorizes a predictor inside the
organism.

## Hard exclusions

Forbidden:

- modifying Frozen Organism v1, M5, M6, or any prior SSA artifact;
- supplying the predictor, application direction, schedule descriptor, basin,
  maturity, or world role to the learner;
- optimizing schedules after observing results;
- learned classifiers, neural predictors, unrestricted decision trees, or
  post-hoc feature invention;
- RNG, probability, sampling, reward, utility, novelty, or exploration state;
- evidence normalization, replay, inverse propensity, or architecture repair;
- definitive evidence, SSA2, scaling, or architecture reopening.

## Stopping rule

Freeze and return after GATE classification, or stop earlier if trace
attribution is scientifically ambiguous. Failed predictor candidates remain
negative; do not tune thresholds, feature definitions, schedules, or accuracy
criteria after evidence.
