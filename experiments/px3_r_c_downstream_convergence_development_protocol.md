# PX3-R Arm C durable downstream-convergence development protocol

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX3 ABSENT**.

## Scope and ancestry

This is one independent mechanism-discrimination development arm. It begins
from exact negative-lineage HEAD
`873094497ff6eb74363191dc5edc479c7d66de72`, whose authoritative PX2 ancestor
is `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`.

The exact first collapse is already frozen at commit
`caff303a5d7e7b4603a49e760fd236a70a41b0a4`, tag
`px3-r-c-downstream-convergence-first-collapse-v1`, in
`experiments/px3_r_c_downstream_convergence_first_collapse.md`. No Arm C CELL
was entered before that freeze.

This protocol authorizes development-only PROBE, MICRO, and GATE. It does not
authorize a definitive command or authority matrix, advance PX3, modify
authoritative PX0-PX2, import another arm, or begin PX4 composition.

## Neutral target

When multiple individually learned routes repeatedly participate in physical
activity that reaches the same ordinary downstream continuation, can ordinary
returned activity and pressure retain shared endpoint convergence that later
distinguishes the recurrent organization from crossed routes with exactly
matched individual marginals?

## Preregistered Arm C physical opportunity

Each fresh cell begins with:

- four physically identical copies of the authoritative learned route motif;
- four ordinary downstream continuation CELLs, each threshold `3` and each
  present before acquisition;
- for every route/continuation combination, one physically ordinary approach
  CELL, threshold `2`;
- a strong ordinary ARROW from the route continuation to each of its four
  approaches, coupling `2` and delay `1`;
- a complete symmetric field of weak approach-to-continuation ARROW
  opportunities, coupling `1`, delay `1`, resistance `1`;
- an ordinary downstream-driver CELL for each continuation, with one strong
  delay-`6`, coupling-`1` ARROW to that continuation;
- ordinary delay-`1`, coupling-`1` returned ARROWs from each continuation to
  the four approaches physically feeding it;
- one ordinary outward ARROW from each continuation into region `1`.

The approach CELL is edge-local and physically identical across the complete
Cartesian field. It is not shared, contains no identifier list, and has no
typed role. It exists because frozen return locality acts at an ARROW's source
CELL. The candidate organization is not an approach CELL: it is the measured
common endpoint of two independently retained ordinary weak ARROWs.

At every acquisition occurrence the environment exposes the complete field,
adding resistance-`1` replacements only where an opportunity is no longer
live. Replenishment is full-field, symmetric, and independent of active route,
active continuation, evaluator scenario, or expected result. Existing live
ARROWs are never reset, replaced, or topped up. This is the sole arm-specific
environmental opportunity.

Actual activity selects structure mechanically. Two active learned routes
send coupling `1` along all their currently weak opportunities. Exactly one
contemporary external downstream-driver SPIKE supplies the third impulse at
one ordinary continuation. That continuation fires, crosses outward, and
returns physical activity only to its own approaches. Only opportunity ARROWs
that actually fired remain eligible at those return arrivals. Frozen local
return adds `3` resistance and may increase coupling from `1` to `2`; frozen
ordinary pressure weakens unsupported and stale opportunities. No evaluator
comparison mutates the graph.

## Frozen cadence and matched marginals

- four acquisition uses per route at ticks `0,16,32,48`;
- first Arm C occurrence tick `64`;
- ten recurrences per acquisition block;
- within each recurrence, first occurrence at `base`, second at `base+8`;
- recurrence bases separated by `16` ticks;
- early/late occurrence order alternates each recurrence;
- A+B physically accompanies continuation `0`; C+D accompanies continuation
  `1` in the initial block;
- every route has ten occurrences, ten traversals, ten consequences, ten
  participation-trace firings, ten returns, and equal outward route effects;
- continuations `0` and `1` each receive ten downstream-driver occurrences;
- held-out observations begin after a `14` tick ordinary-pressure gap on
  complete-state clones and supply route activity only, with no downstream
  driver and no opportunity replenishment;
- trained observations are A+B and C+D; crossed observations are A+D and C+B;
- individual observations are A, B, C, and D separately.

The evaluator serializes correspondence resistance and learned direction
resistance for A, B, C, and D separately from the `4 x 4` opportunity
resistance/live/coupling matrices and common-endpoint overlap scores. A
positive requires exact per-route equality; it cannot be explained by an
individually stronger route.

## Ordered PROBE

The PROBE uses namespace `0x9_4300_0000` and an exact duplicate replay. It
passes only if:

1. all frozen-source, ancestry, clean-artifact, and forbidden-information
   preconditions pass before entering a CELL;
2. route marginals and separately serialized route strengths are exact;
3. both trained organizations produce exactly one downstream outward crossing
   apiece on held-out route-only use;
4. both crossed organizations and all four individual routes produce zero
   downstream outward crossings;
5. the trained routes have a common live endpoint with coupling `2`, while
   each crossed combination lacks any common live endpoint at threshold;
6. correlation without route participation, participation without returned
   activity, and absent opportunity produce no reusable convergence;
7. every finite execution is naturally quiescent, has zero autonomous route
   source refiring, and exact duplicate replay.

The first failed clause is frozen. MICRO is forbidden after PROBE failure.

## Ordered MICRO

MICRO uses fresh namespaces beginning `0x9_5300_0000`. It first repeats the
PROBE result, then swaps contemporary recurrence to A+D with continuation `0`
and C+B with continuation `1`, with the same ten-recurrence cadence and full
matched marginals.

It passes only if old A+B/C+D common-endpoint overlap weakens under ordinary
activity and pressure, new A+D/C+B overlap emerges, new trained held-out use
crosses, and the now-crossed old combinations do not. No old ARROW is deleted
or reset by the evaluator.

Isolated controls additionally require:

- correlation without actual route participation: no retained structure;
- actual route participation without continuation return: no retained
  structure;
- ambiguous three-route activity with no unique downstream driver: no
  arbitrarily selected single structure;
- absent opportunity: no structure;
- stale structure after an ordinary `220` tick pressure gap: no crossing;
- a blocked opportunity field: no crossing;
- four genuinely experienced stable organizations A+B->0, C+D->1, A+D->2,
  and C+B->3, presented in a balanced cycle, may all remain reusable.

The first failed clause is frozen. GATE is forbidden after MICRO failure.

## Ordered development GATE

GATE uses four fresh namespaces beginning `0x9_6300_0000`; this is a focused
development gate, not an authority matrix. The four cells vary, independently
of expected organization:

- physical identities;
- mirrored spatial layout;
- normal versus reverse CELL allocation;
- normal versus reverse same-tick arrival insertion;
- within-recurrence spacing `8` versus `11`;
- route/continuation physical-ID permutation;
- `0`, `8`, `16`, or `24` inert distractor CELLs with matched external
  activity;
- original organization followed by the preregistered swap;
- duplicate complete-state replay.

All PROBE and MICRO clauses must hold in every GATE cell. Exact raw physical
fingerprints may differ with identity/layout; relation-normalized physical
matrices and behavior must agree. All queues must drain naturally and no
autonomous source refiring is allowed.

## Commands, markers, and atomic artifacts

After a separately frozen implementation passes no-CELL preflight, the only
authorized stage commands are, once each and in order:

```text
cargo run --release -p px0-physical-correspondence \
  --example px3_r_c_downstream_convergence -- --probe
cargo run --release -p px0-physical-correspondence \
  --example px3_r_c_downstream_convergence -- --micro
cargo run --release -p px0-physical-correspondence \
  --example px3_r_c_downstream_convergence -- --gate
```

Each prints one stage-specific `PX3_R_C_*_DEVELOPMENT_EVIDENCE_SPENT` marker
before entering its first CELL. Final paths are:

- `results/px3_r_c_downstream_convergence_probe_v1.csv` and `.md`;
- `results/px3_r_c_downstream_convergence_micro_v1.csv` and `.md`;
- `results/px3_r_c_downstream_convergence_gate_v1.csv` and `.md`.

Each stage writes same-directory hidden staging files with exclusive creation,
flushes them, then atomically renames them only after the report is complete.
Existing final or staging paths cause refusal before execution. No stage is
rerun, rescued, regenerated, or tuned after its evidence marker.

## Forbidden-information audit

Between explicit organism-visible source markers, case-insensitive source must
contain none of: Event, Episode, History, Pair, Group, member, pair key,
semantic, boundary, old M3, adapter, co-occurrence record, scenario, expected,
evaluator, trained, crossed, or renamed equivalents. Organism-visible state
may contain only CELL/ARROW/SPIKE substrate physics, numeric physical IDs,
positions, thresholds, coupling, resistance, local time, and work counters.

Evaluator labels and expected organizations exist only after the marker. They
may schedule external physical arrivals and read measurements, but have no
path to add a non-symmetric opportunity, choose an update, delete structure,
or feed a comparison into the substrate.

## Stop rule and handoff classification

- first mechanical failure: freeze a development negative and stop;
- representation/substrate-law need beyond the preregistered opportunity:
  freeze unresolved ambiguity and stop;
- all three stages pass: freeze a **positive development candidate** and stop
  at development readiness.

No outcome is PX3 authority or permission for definitive evidence.
