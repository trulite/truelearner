# SSA1-C2 lock-in / hysteresis map — development classification handoff

## Outcome

**Classification A — finite reversal barrier**, with a conjunctive B-only
subclassification:

> **Absorbing for B-only same-class counterexperience.**

Frozen Organism v1 is not globally irreversible. A mature locked landscape can
reverse under ordinary paired changed-world experience, but it cannot reverse
from arbitrarily repeated B-only evidence of the tested stable consequence
class. The difference is whether experience also changes the old A consequence
history enough to make A lose M6 eligibility.

This result does not amend SSA1 or SSA1-C1, does not start SSA2, does not alter
Frozen Organism v1, and did not execute definitive evidence.

## Six-cell GATE result

All six fresh cells, alternating A/B identity and layout, produced the same
map. The GATE CSV contains 972 preregistered checkpoints plus its header. Every
physical resolution was duplicate-exact.

### Maturation map

| Initial stable-A history | Boundary | B-only result | Paired changed-world result |
|---:|---|---|---|
| 0--2 | M6 A not yet eligible | B dominates by 10,000; first recorded dominance at 64 | B dominates; first recorded dominance at 64 |
| 4 | M6 A eligible; M5 not yet sealing | both routes remain `[4,4]`; no B dominance | B dominates by 1,024 |
| 6--32 | A-favored allocation develops | B-only reaches/keeps `[4,1]` | B dominates by 1,024 |
| 64 | physically sealed `[4,1]` | invariant through 10,000 | B dominates by 10,000 |
| 192 | physically sealed `[4,1]` | invariant through 60,000 | B dominates by 10,000 |

The precise empirical transition is therefore conditional, not a single
irreversible 8-to-16 boundary. Early history controls how much changed-world
experience is required:

```text
H <= 2       reversal recorded by 64
H = 4..32    reversal recorded by 1,024
H = 64..192  reversal recorded by 10,000
```

The checkpoints bound the transition; they do not claim that reversal occurs
exactly at those episode numbers.

## Exact B-only absorber

At mature H=`192`, route B was physically enabled and executed 60,000 times.
Every execution returned one ordinary stable downstream consequence. The
frozen runtime state followed this chain:

```text
initial A evidence:
  M6 support/margin = 192/192
  A eligible = true

B observations 1..3:
  A eligible, B ineligible
  -> M6 selects A
  -> active B receives three negative M5 differentials
  -> B M5 support/rejection = 0/3

B observation 4 onward:
  A eligible, B eligible
  -> M6 abstains
  -> no further M5 differential
```

At B observation 60,000:

- M6 B evidence: 60,000 observations, support 60,000, margin 60,000;
- M6 A remained eligible;
- M6 abstentions: 60,000;
- B M5 support/rejection remained `0/3`;
- landscape remained `[4,1]` (or its exact mirror);
- B could not execute independently after background removal.

The frozen M6 source has no ordinary evidence pressure or decay path. Once
both encounters are eligible, additional evidence of the same stable B shape
cannot change the `[true, true] -> abstain` branch. This establishes an exact
absorber for the preregistered B-only evidence class, not merely a slow result
through 60,000 observations.

## Why the paired changed world reverses

The paired schedule supplied one physically enabled stable-B opportunity and
one ordinary varying-A opportunity per unit. No unexecuted route received
evidence.

At mature H=`192`, the route-0 example showed:

### Budget 1,024

- A evidence: 1,216 observations; leading support 512; margin 0; **ineligible**;
- B evidence: 1,024 observations; support/margin 1,024; **eligible**;
- B M5 support/rejection: `643/3`, score `640`;
- A M5 support/rejection: `192/644`, score `-452`;
- landscape had reopened to `[4,4]`, but A had not yet fallen below physical
  threshold, so dominance reversal was not yet claimed.

### Budget 10,000

- A evidence: 5,430 observations; leading support 2,619; margin 0;
  **ineligible**;
- B evidence: 14,762 observations; support/margin 14,762; **eligible**;
- B M5 support/rejection: `14,381/3`;
- A M5 score: `-4,666`;
- landscape: `[1,4]`;
- independent physical replay realized B and not A.

Thus reversal occurs through the unchanged learned path:

```text
continued varying A experience
  -> A's M6 consequence margin collapses
  -> A becomes ineligible

continued stable B experience
  -> B remains M6-eligible
  -> M6 emits B-favoring differentials
  -> M5 value and allocation move
  -> B supporters form and A supporters decay
  -> [4,1] becomes [1,4]
```

The organism is hysteretic: later evidence is interpreted through mature
credit state. But it is reversible when the changed experience alters both the
new hypothesis and the old hypothesis's continuing consequence regularity.

## Physical-support and disuse maps

At mature H=`192`:

- 0, 1, or 2 additional early B spikes: B executed zero times; first barrier
  was physical execution.
- 3 additional early B spikes: B executed 10,000/10,000 times; first barrier
  moved to M6 credit.
- equivalent late spikes were inert;
- stale/blocked B could not win.

Ordinary pressure through 1,024 no-encounter events did not reopen A's mature
allocation. At T=`1,024`, the sparse B proposal had disappeared (`[4,0]`), but
A remained live and M6 A evidence remained exact. Subsequent B-only exposure
restored only the sparse `[4,1]` state. No forgetting-only reversal occurred in
the tested T range.

## Frozen count-capacity boundary

Pre-execution audit found that M6 stores each normalized consequence-shape
count as `u16`. The experiment therefore did not run a single-shape 100,000
trajectory or rely on integer wraparound.

- E0 safe anchor: 60,000 B-only opportunities, at most one B observation each.
- E1 safe anchor: 30,000 pairs, at most two B observations each.

All recorded single-shape counts remained below the frozen capacity. The exact
B-only source invariant, rather than extrapolation from a wrapped counter,
supports the absorber claim beyond the empirical anchor for that evidence
class.

## Validation and exclusions

Passed:

- six fresh seeds with alternating A/B identity;
- complete 13-point maturation grid;
- both evidence schedules and all safe anchors;
- four physical-support levels and five disuse levels;
- 972 GATE checkpoints;
- exact complete-state duplicate replay at every physical resolution;
- stale/blocked and post-closure controls;
- frozen source invariant audit;
- focused M8 development replay (`5/5` tests);
- formatting and C2 test-surface compilation.

Literal strict Clippy stopped on the three established frozen warning classes
plus evaluator-only `needless_range_loop` and `too_many_arguments` structure.
Strict Clippy passed with those documented classes allowlisted; no frozen source
was edited to satisfy lint. The development runner refused `--definitive` with
exit 2.

Forbidden and absent:

- Frozen Organism v1, M5, M6, SSA0.3, SSA1, or SSA1-C1 changes;
- evidence decay, replay, reopening state, or a new learning rule;
- evaluator-selected credit, winner, plastic site, or proposal;
- counterfactual evidence for unexecuted routes;
- RNG, noise, probability, novelty, or exploration signals;
- SSA2, scaling, definitive evidence, or architecture reopening.

## Frozen commits and tags

| stage | commit | tag |
|---|---|---|
| final protocol | `0d7e1a5f8e265475b06279722d5ac886efe5cff7` | `ssa1-c2-lock-in-hysteresis-map-protocol-v5` |
| executed implementation | `6a3c01493bad4c8119c9f80690e3ec7e4c32d423` | `ssa1-c2-lock-in-hysteresis-map-implementation-v4` |
| PROBE v1 diagnostic | `ba508c008ee6236f0d14ebedd5dd6fbd3611b0a3` | `ssa1-c2-lock-in-hysteresis-map-probe-v1-classifier-diagnostic` |
| PROBE result | `af62989d9d492cb3b596728e6595c780e1a1caae` | `ssa1-c2-lock-in-hysteresis-map-probe-v2-credit-edge` |
| MICRO | `49923cf22d6effd78d14015bc2bb7c106e99fd79` | `ssa1-c2-lock-in-hysteresis-map-micro-v1-classification-a` |
| GATE | `226f8d356fef490cbad51f5bca058036d23bdd40` | `ssa1-c2-lock-in-hysteresis-map-gate-v1-classification-a` |

## Artifact hashes

| artifact | SHA-256 |
|---|---|
| protocol | `3104138ea5c07546e4254b3b2557d59e23b89b79810ae88fecd8f524bec3af40` |
| evaluator | `c7a785763d9283bd213a951a7c0fd378d8d9b63a3e3717cf51d250ff25ce6a8d` |
| audit adapter | `572de33f00e80f237a3e23ded54759adb57002c9c776f7f2f180f31bd10c5cc2` |
| runner | `74a5a4cb7d8f96012e44111a66562c8170f857678390ef9bdb610af49c7a8c10` |
| PROBE | `5ab8bed6ab3c6595e0dbff63643f93f1f7283546952adedd2ba308a93d3733b8` |
| MICRO | `65e0b112f3b8f8ec11323424158f5e17e2f60ff84f82f01e2d20882430295694` |
| GATE report | `21e9709ff3f1a98f4212b82811719ed6f1db3ec7f8c35b0ccd9ba4df45d5aede` |
| GATE CSV | `2fbae8cb98f9809230317a79f42c3991600df9cda0d58ac64763a3554fa607ba` |

Frozen parent hashes remained exact:

- substrate: `e49578f050f75fe0be181930d6231815abdbdc382b1b5b8c690cb19a637b68d3`
- SSA1: `dc157e0bd238992d6475e5dc9767c6f7711a1bb5b7759ebdb7991573aea5199b`
- SSA1-C1: `bbead342b9bb51b47efbaae87483a4114b127779e0d19f68a4632b89e78b1602`
- M5: `e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7`
- M6: `11b4229122b3e0788ca30c55579b91ffe07461de9a138860690134565fcf2ed6`

## Program state

```text
Frozen Organism v1    unchanged
SSA1                   Classification C — collapse/preservation only
SSA1-C1                Classification C — curriculum prevention only
SSA1-C2                Classification A — finite reversal barrier
                        + B-only absorbing credit state
SSA2                   blocked
architecture change    not authorized
```

The narrow answer is now:

> Frozen Organism v1 has reversible learning under a sufficiently rich changed
> world, but a mature M6 state is absorbing for counterexperience that only
> strengthens the alternative while leaving the old consequence regularity
> intact.
