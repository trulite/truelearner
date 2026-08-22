# Cumulative DS3 event/container-boundary port protocol

Protocol identifier: `ds3-cumulative-event-boundary-v1`

Status: **PREREGISTERED BEFORE PORT IMPLEMENTATION**. This protocol freezes
the wiring contract between the authoritative M2 parent and the byte-frozen
isolated DS3 mechanism before any learner-facing code is written or run.

Parent chain:

```text
expectation freeze 8ca36f5 tag ds3-cumulative-event-boundary-expectation
authoritative M2    162a5b2082a8c1ac9ede45bc5178fecf3509b476
```

## Question

> Does byte-identical isolated DS3 compose directly onto M2 when its two
> formerly supplied inputs are replaced only by already-existing learned M2
> counterparts?

## Frozen mechanism and permitted edits

The learner core of isolated DS3 is frozen byte-for-byte at source SHA-256
`a8d8fe060b497c7a6b5f9a5a88b7ed2292dc8a729a8781f599547b6027efc0a0`
(branch `bb/ds3-isolated-event-boundary-de-supply-thr_wpuyfghsb9`). Frozen
region: everything from the `CONSOLIDATION_SUPPORT` constant through the end
of `impl BoundaryLearner`, plus `check_predictions`, `signature`, and
`to_span`. This includes `ChunkSignature`, the persistent-state block between
`DS3_PERSISTENT_START`/`DS3_PERSISTENT_END`, all transition guards, the two
local role/causal checks per observation, support threshold 2, prediction
validation, invalidation/reopening behavior, and work counters.

Permitted edits outside the frozen region only:

- evidence-interface wiring producing `Observation.role/link` values from M2
  learned outputs;
- evaluator-side fixture construction, reporting, matrix enumeration, control
  plumbing, artifact paths;
- mechanical type/module-path changes proven behavior-neutral.

Any change inside the frozen region is a new mechanism and ends this lane as a
protocol violation, not a result.

## Wiring contract

Every organism-visible observation value is derived from learned M2 lineage
machinery running on ordinary activity. Evaluator span knowledge never enters
role/link derivation.

| Isolated input | Learned M2 counterpart | Source machinery |
|---|---|---|
| role `Open` | first probation observation of a new route proposal | A1 `Learner::observe` first sight of a `LocalTemplate` |
| role `Continue` | further probation observations extending support below consolidation | A1 `Learner::observe` before `SUPPORT_EPISODES` |
| role `Close` | consolidation: root installed and exposed | A1 `install` reaching `SUPPORT_EPISODES`, `structural_dedup`, `expose_roots` |
| role `Singleton` | single reactivation of an already-consolidated retained asset | IR0 historical-return use of a retained route |
| role `Interrupt` | actuation attempt on a stale/blocked handle that abstains mid-route | AC0 stale-handle/blocked-route abstention |
| link `Reset` | fresh-direction opening after generic acquisition or generic reopening | IR0 reopened execution; fresh-substrate installation after acquisition |
| link `Continue` | retained-direction execution structurally matches the learned shape | IR0/RT0 `structural_match` path |
| link `Broken` | retained-direction execution structurally mismatches; route invalidated | IR0 structural mismatch -> invalidation |
| `functional_relation` | learned route class from the executed effect | A1/RT0 `NormalizedEffect`/`RouteShape` class |
| `propagation` | physical propagation class of the executed route | arrow traversals / observed propagation edges |

If any row has no organic counterpart in practice, the corresponding control
fails and the lane freezes the lettered collapse point from the expectation
freeze (`C1`..`C5`) without repair.

Explicitly forbidden channels: evaluator membership or grouping truth in any
wiring function; container/span/event IDs under any name; thresholds,
signatures, guards, or persistence altered to make composition succeed;
restoring a removed scaffold through the fixture generator.

## Fixtures

Episodes are produced by running M2-lineage machinery (E0 formation, A1
probation, RT0 retention, IR0 lifecycle) over ordinary seeded activity. The
evaluator separately records expected spans for scoring only. Streams used for
controls are perturbed mechanically (blocked handles, mismatched signals,
permuted identities, shuffled timing) through the same public machinery
options, not by editing streams by hand where a machinery option exists.

## Controls

The twelve isolated controls carry over with cumulative equivalents, all
conjunctive and reported independently:

1. identical local shapes, different grouping (learned output follows
   substrate lifecycle evidence, not shape runs);
2. different shapes, same functional span (identity relabeling transfers);
3. boundary shifts (lifecycle evidence moves the cut; old cut does not
   persist);
4. interruptions and re-entry (stale handles interrupt; re-entry completes
   only from the new opening);
5. shuffled timing / relabeled consequences are not grouping keys;
6. fresh occurrence identities and allocation orders transfer;
7. leak/source audit of the frozen persistent region;
8. invalidation and generic reopening followed by reacquisition;
9. subthreshold recurrence installs no chunk;
10. missing close fails closed;
11. invalid causal transition (reset without opening) fails closed;
12. held-out population enforcement.

Plus the parent-audit stage verifying the exact M2 ancestor and the frozen
isolated mechanism hashes before any probe runs.

## Modes

- MICRO: seed namespace `83_000`, two acquisition and two fresh evaluation
  episodes per arm. Development only.
- GATE: seed namespace `84_000`, six acquisition and eight fresh held-out
  episodes per required control, full audit set, duplicate determinism, work
  attribution. Development only.
- DEFINITIVE: locked in this lane. A separate matrix preregistration (seeds,
  cells, contexts) must be committed and tagged before the single write-once
  definitive execution is authorized. No definitive artifact exists before
  then; `--definitive` refuses during development modes.

Neither MICRO nor GATE writes anything under `results/`.

## First-collapse rule

Stages evaluate strictly in order; the first failure freezes the result:

```text
P0 parent and mechanism hash audit
P1 wiring produces legal role/link streams from learned machinery alone
P2 reconstruction on held-out streams
P3 functional adequacy (consequence parity)
P4 controls 1-12
P5 duplicate determinism and work attribution
```

A failure at P1-P5 maps to the expectation-freeze classification (`C1`..`C5`)
at the exact failing control; every later stage is `BLOCKED`. Freeze
vocabulary: `DS3-CUMULATIVE DEVELOPMENT READY` or
`DS3-CUMULATIVE COLLAPSE AT <stage/control>`; only a separately authorized
definitive run can produce `CUMULATIVE POSITIVE` or `CUMULATIVE NEGATIVE`.

## Execution environment

All Rust formatting, compilation, Clippy, tests, MICRO, GATE, and DEFINITIVE
execution use the shared runner
`truelearner/scripts/e2b_persistent.py` with credentials
`truelearner/.env.e2b` and the dedicated state file
`/Users/satya/.cache/truelearner/ds3-cumulative-e2b.json`. The dedicated
sandbox is reused, never killed, and left running. Local commands inspect
text, Git state, and hashes only.
