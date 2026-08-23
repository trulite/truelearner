# CJ0 ARM CJ-B locally gated ARROW frozen-negative handoff

Status: **TERMINAL DEVELOPMENT NEGATIVE; PROBE/MICRO POSITIVE; GATE NEGATIVE**.

## Outcome and classification

CJ-B found one minimal no-new-variable physical conjunction candidate. It
passed PROBE and mandatory reversal/bootstrap MICRO, including self-evidence,
matched trained-versus-crossed discrimination, forgetting/relearning, and
full-deallocation formation. It then failed the terminal GATE in one exact
timing stratum.

The lane outcome is therefore a **frozen development negative**, not a viable
handoff candidate and not a PX3 advance. The exact first failure is a weak
first-use delivery/pressure race: changed C+B produced `19` rather than `20`
training outward effects in all four S0 rows. S1 produced `20`. Final changed
reuse, old deallocation, recursion, convergence, temporal expressivity,
replay, and quiescence all passed.

The failure is mechanically unique and does not create an architectural
choice, but the preregistered terminal protocol forbids rerun or rescue. The
hard boundary also ends the lane at GATE. PX3, PX-C, and PX4--PX8 remain in
their prior states.

## Frozen start and preserved negatives

- exact start/authoritative ancestor:
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`, tag
  `px2-physical-causal-direction-authoritative`;
- authoritative PX0--PX2 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- authoritative PX2 execution SHA-256:
  `c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5`;
- frozen PX3 Class-D and PX3-R Arms A/B/C: unchanged, not rerun, not
  reinterpreted, and not copied into this arm;
- existing-state insufficiency audit commit/tag:
  `5e614f3`, `cj0-b-existing-state-negative-audit-v1`.

The complete diff from the frozen start contains 25 added files and no
modified/deleted/renamed pre-existing path.

## Exact physical law

Physical source:
`arms/cj-b-locally-gated-arrow/src/lib.rs`, SHA-256
`ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188`.

For every live outgoing ARROW at current tick `t`:

```text
decay(destination.state, t)
available = destination.state + arrow.coupling

available < destination.threshold:
    emit nothing; create no eligibility

available >= destination.threshold:
    consume destination.state := 0
    emit one SPIKE(impulse = available)
    expose only this traversed ARROW to ordinary local return
```

Strong ordinary ARROWs whose coupling alone reaches threshold propagate
normally. Weak coupling-1 candidates need current contributor state 2 to
reach threshold 3. Returned activity can raise coupling only to 2, so a
mature source alone remains insufficient. Generic local proposal recreates a
missing coupling-1 ARROW after deallocation; no mature higher-order firing is
required to bootstrap.

Persistent state adds no field. It contains only ordinary numeric CELL,
ARROW, SPIKE, local time/order, threshold, coupling, resistance, generation,
liveness, refractory, eligibility, and queue state. There is no gate flag,
contributor key, member list, relation record, semantic adapter, logical
operator, evaluator feedback, or renamed old schema.

## Consume/produce contract

Consumes physically:

- actual current positive destination CELL state left by ordinary contributor
  participation;
- actual source CELL firing and its local outgoing ARROW;
- decayed local time, numeric coupling/threshold, liveness/generation;
- actual returned activity, ordinary pressure, and generic local proposal.

Produces physically:

- either no transmission/eligibility, or one ordinary SPIKE carrying the
  numeric local sum after consuming current destination state;
- destination firing/outward propagation if delivery remains live;
- route-local return eligibility and reusable resistance/coupling only after
  actual traversal;
- physical weakening, deallocation, new generations, and generic reproposal
  under changed experience.

An old learned ARROW activated by its source alone sees state 0, suppresses,
and cannot manufacture fresh evidence.

## Phase results and hashes

### PROBE — positive

- implementation commit/tag: `f637b58`,
  `cj0-b-locally-gated-arrow-probe-v2-implementation`;
- evaluator SHA-256:
  `f8df9f19e76ff800ddcbd24a4eb7be7743c208dc30bfc6be962d6a5de9ce57ca`;
- result commit/tag: `6b94574`,
  `cj0-b-locally-gated-arrow-probe-v1-positive`;
- CSV/report SHA-256:
  `297086aaacc6b6ef1d099e4887adbe49823c65f972b9b7a63a1096cc8faa7611` /
  `eb90ba3002ff1e392802cbe32935fc7f1afa1e139a1739803c5558294c94cd1c`;
- result: `4/4` rows, `40/40` claims;
- trained held-out `1|1`, crossed `0|0`, A/B/C/D singletons
  `0|0|0|0`, self-evidence consumption `0`;
- too-late, correlation-only, no-return, absent, blocked, stale, ambiguity,
  replay, and quiescence controls passed.

The v1 evaluator tag is preserved. Before evidence, a separately frozen v2
accounting amendment corrected only candidate-consumption projection; the
physical module never changed.

### MICRO — positive

- implementation commit/tag: `983f6ec`,
  `cj0-b-locally-gated-arrow-micro-v1-implementation`;
- evaluator SHA-256:
  `d9d9c747fa655b2cfc76fa9b7e06329faee67542612bff3ce2f575ab80fa1a6c`;
- result commit/tag: `2de3301`,
  `cj0-b-locally-gated-arrow-micro-v1-positive`;
- CSV/report SHA-256:
  `d88aea36480022740067148431afbfe4b8150c729d476670b8b4dcaf53c4a2fa` /
  `bb7ea15b79ec0dbbd6d77591b2f0adf7a8bd44661923d6bae9a4e0669e13a871`;
- result: `4/4` rows, `40/40` claims;
- old pre-change held-out `1|1`; new post-change `1|1`; old immediate/late
  `0|0|0|0`;
- old live resistance `0|0`; new live resistance/coupling `38|38` / `2|2`;
- full-deallocation bootstrap `1|1`; identical no-return reuse `0|0`;
- no evaluator invalidation, relation-change signal, or historical
  resurrection.

### GATE — frozen negative

- implementation commit/tag: `dd9390f`,
  `cj0-b-locally-gated-arrow-gate-v1-implementation`;
- evaluator SHA-256:
  `84788baec691de2eb5bdce24f19c4456c43ce40717a93edbc11cf42a3e99d61d`;
- result commit/tag: `527a751`,
  `cj0-b-locally-gated-arrow-gate-v1-frozen-negative`;
- CSV/report SHA-256:
  `e76f7256033de9352fac76de72c5ff37a8dcec899c2ff5eec3447d9286604707` /
  `08394d47aa7c7c494cc5d5b266868780ecba6009950db1f63eb50112bed1a1d8`;
- result: `4/8` rows, `76/80` claims; only `G3` false in four S0 rows.

The exact failure sequence is proposal at tick 59, delay-2 delivery due at 61,
ordinary tick-60 pressure deallocation of the resistance-1 ARROW, then queued
generation mismatch. Later changed occurrences mature replacement structure,
so terminal held-out behavior remains correct. The S1 schedule begins after
its pressure boundary and passes `20|20`.

## Recursion, convergence, and temporal results

All eight GATE rows passed these surfaces despite the flat G3 failure:

- same-law recursion: A+B->X, X+C->Y, Y+D->Z `1|1|1`;
- full chain X/Y/Z `1|1|1`, one Z outward crossing;
- missing B/C/D downstream crossing `0|0|0`;
- all three recursive ARROWs resistance `20`, coupling `2`;
- ordinary convergent A-only, B-only, A+B activation `1|1|1`, with no added
  disjunction mechanism;
- temporal crossing signature together/A-then-B/overlap/within/absent
  `1|0|1|1|0`;
- together `impulse 4 @ tick 0`, overlap `4 @ 1` with two contributor
  arrivals, within-window `3 @ 1`, and distinct no-output transient
  fingerprints for A-then-B versus contributor-absent closure;
- exact replay, finite recurrence, and natural quiescence: pass.

## Validation, work, and storage

- standalone crate dependencies: zero;
- physical focused unit tests: `2` pass, `0` fail;
- format and strict all-target Clippy: pass;
- all three no-CELL preflights: pass;
- no-argument and neutral wrong-argument refusal: exit `2`;
- hard-boundary disclosure: the earlier rejected `--definitive` refusal check
  entered zero cells and wrote no marker/artifact; no such scientific surface
  exists;
- physical forbidden-vocabulary scan: empty;
- result shapes: PROBE `4 x 75`, MICRO `4 x 68`, GATE `8 x 94` plus headers;
- all staging paths: absent;
- ledgered physical work: PROBE `12,984`, MICRO `55,032`, GATE `92,852`,
  cumulative `160,868` operations;
- atomic result storage: `12,352` bytes;
- lane-added tracked storage before this handoff: `181,503` bytes in 25 files;
- authoritative files modified: none.

## Stop and blocker

The candidate is not eligible to advance because the terminal exact GATE is
negative. The lane's hard boundary and frozen no-rerun protocol prohibit a
timing-aligned correction or another GATE. No additional persistent variable,
semantic representation, PX0--PX2 change, or architectural choice was shown
necessary; nevertheless there is no positive terminal candidate.

This handoff is the final lane artifact. Its commit and annotated handoff tag
are created next, followed by branch/tag publication and local/remote equality
verification.
