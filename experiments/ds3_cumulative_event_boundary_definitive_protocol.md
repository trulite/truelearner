# DS3 cumulative event-boundary definitive protocol

Status: **PREREGISTERED; SINGLE DEFINITIVE OUTCOME UNSPENT**.

Exact frozen development parent:
`8e24a1316327f0af40fa3e7c70ad940d2a3e203f` /
`ds3-cumulative-event-boundary-development-readiness`.

The development ancestor is immutable. No source, threshold, evidence channel,
fixture behavior, control, or interpretation in that ancestor may change.
Definitive work must be a separate wrapper and write-once serializer over the
tagged parent.

## Question

> On a fresh, fixed, held-out population, does the byte-frozen isolated DS3
> mechanism reconstruct event/container boundaries when every organism-visible
> role and causal input comes only from the already-learned M2 correspondence,
> interaction-role, route-probation, physical-execution, and
> invalidation/reopening machinery?

The claim is deliberately narrow. A positive result establishes learned event
organization over the frozen M2 substrate. It does not establish language,
general planning, persistence, semantic credit, or unbounded scaling.

## Frozen lineage and hashes

The definitive wrapper must verify all of these before any cell runs:

```text
authoritative M2       162a5b2082a8c1ac9ede45bc5178fecf3509b476
development parent     8e24a1316327f0af40fa3e7c70ad940d2a3e203f
expectation             8ca36f5b44f57f675057307783cae3bc984b641a
port protocol           1878c018e520cae8cac9e1af229f03f87831a9b5
mechanism install       6d3fea34e13b1417356f76cbf04e9d9916ec61fb
development code        9ab6824963f3c890e1ca457bcc96ad5b6dd34d7c
development handoff     75c8f734cd0b7c958b84e252be15a376541bfab7ef2b4df60029ce29f596b321
port source             c4fc7aca11a5925effeb5a84b90184a70da0f66da7c063d0f87ba46ca36addf3
development runner      0af64854568b85ffb6ab6b6cdd55dc0759e4323ebd303502107c06f718e79ec4
frozen DS3 mechanism    a8d8fe060b497c7a6b5f9a5a88b7ed2292dc8a729a8781f599547b6027efc0a0
A1 source               b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1
AC0 source              860e89304e86f254dd02a5aa35cf63cc240af160039b4166fa0cb5856dacb84a
IR0 source              f81cc694f2d6d9e43cb04e8d1a1db301687e6644899665ae470abed1f9e4a7dc
```

The wrapper must also verify that the six-stage development report remains
entirely `READY`, all twelve development controls remain present in their
frozen order, the development runner still rejects `--definitive`, and no
pre-existing cumulative DS3 definitive artifact exists.

## Frozen matrix

The matrix contains exactly sixteen independent, single-threaded blank-start
cells. Logical cell IDs are `0..15`. Their fresh base seeds are:

```text
1_000_000 + cell_id * 100_000
```

Thus the exact base seeds are `1_000_000`, `1_100_000`, ..., `2_500_000`.
Their 100,000-wide namespaces are disjoint from one another and from the
development MICRO/GATE namespaces `83_000` and `84_000`.

Every cell uses a fresh DS3 learner and the unchanged M2-to-DS3 wiring.

### Acquisition population

- exactly 8 acquisition streams per cell;
- acquisition stream seeds are `base + 0..7`;
- every stream contains one complete organic A1 probation/install/expose/
  execute lifecycle for route 0 and one for route 1;
- each lifecycle produces `Open/Reset`, `Continue/Continue`, and
  `Close/Continue` only through the frozen wiring;
- acquisition is enabled only for these 8 streams;
- no control mutation or evaluator expected span is acquired.

Expected per-cell acquisition accounting is exact:

```text
M2 organism work                 6_872
DS3 acquisition observations       48
DS3 candidate comparisons           16
```

### Held-out population

- exactly 16 fresh, read-only held-out streams per cell;
- held-out seeds are `base + 10_000 + 0..15`;
- each stream contains both complete routes and therefore exactly two expected
  spans;
- no held-out call may mutate DS3 support or chunks;
- the four perturbation rows below repeat exactly four times, in order:

| Held-out ordinal modulo 4 | Identity relabel | Reverse allocation | Shape transform | Local time | Consequence |
|---:|:---:|:---:|---|---|---|
| 0 | no | no | unchanged | forward | unchanged |
| 1 | yes | no | XOR `0xA7` | forward | unchanged |
| 2 | no | yes | unchanged | reversed | unchanged |
| 3 | yes | yes | XOR `0xA7` | reversed | wrapping `+31` |

Every cell must report exact reconstruction and consequence parity for all
32/32 held-out spans, 32 learned-signature uses, zero held-out acquisition
observations, unchanged chunks/support across held-out evaluation, and:

```text
generic mature work      256
learned mature work      128
DS3 chunks                 2
DS3 persistent bytes      20
```

## Frozen context/control battery

Each cell independently runs all twelve controls after held-out evaluation.
Control streams are read-only except controls 9 and 10, which use their own
fresh throwaway learners. The exact control seed namespace is
`base + 20_000..=base + 30_000`; no control seed may enter acquisition.

1. **Identical local shapes, different grouping.** Compare a flat-shape-9
   standard two-lifecycle stream with a flat-shape-9 retained singleton plus
   probation lifecycle. Both reconstruct; their span layouts differ.
2. **Different shapes, same functional spans.** Apply identity relabelling,
   shape XOR `0xA7`, reverse allocation, and consequence `+31`; the two learned
   spans remain exact.
3. **Boundary shift.** The retained-use plus probation stream reconstructs
   exact span lengths `[1, 3]`; the old `[3, 3]` cut is absent.
4. **Interruption and re-entry.** Run both a stale-handle and a blocked-route
   AC0 abstention after `Open, Continue`. Each produces `Interrupt/Broken`; no
   partial span completes, and only the fresh re-entry opening completes.
5. **Timing and consequence are not grouping keys.** Reverse local time and
   apply consequence `+31`; span indices and roles remain exact and the changed
   ordinary consequence propagates exactly.
6. **Fresh identity/allocation transfer.** Relabel E0 occurrence identities and
   reverse allocation; learned signatures are reused with no acquisition.
7. **Leak/source audit.** All hashes and frozen A1, AC0, IR0, and isolated DS3
   source audits pass. The DS3 persistent region retains no occurrence,
   evaluator grouping, container/event ID, time, or consequence field.
8. **Invalidation and generic reopening.** For original route
   `cell_id modulo 2`, run both ordinary and relabelled/reverse-layout IR0
   changed-dependency lifecycles. Each reports exactly one compatible use,
   structural mismatch, invalidation, invalidated abstention, generic
   reopening, reopened execution, and historical return, with unchanged
   historical support.
9. **Subthreshold recurrence.** One complete signature presentation to a fresh
   DS3 learner leaves exactly zero chunks.
10. **Missing close.** Two A1 probation presentations produce only
    `Open, Continue`; DS3 completes zero spans.
11. **Invalid causal transition.** In a control-only, never-acquired stream,
    replace the first continuation link with `Reset`. The learned candidate
    invalidates and fewer than the expected spans complete.
12. **Held-out population enforcement.** Acquisition, held-out, and control
    namespaces are pairwise disjoint; evaluation performs no acquisition and
    leaves chunk/support state byte-identical.

All twelve controls are conjunctive. A control failure freezes the exact
control number and blocks later controls in the reported interpretation even
if their diagnostic code ran.

## Duplicate replay and cell isolation

Every complete cell is executed twice from separate blank learners. The two
in-memory cell reports, including source audit, spans, work counters, controls,
first-collapse value, chunks, and persistent bytes, must be byte-identical.
Only the first copy enters the definitive matrix artifact. Cells run in logical
ID order, single-threaded, and share no learner or mutable fixture state.

## Ordered outcome rule

Each cell evaluates the frozen stages in this order:

```text
P0 exact parent, hashes, source, and matrix dimensions
P1 legal learned M2 role/link wiring
P2 held-out event-boundary reconstruction
P3 ordinary-consequence parity
P4 controls 1..12 in order
P5 duplicate determinism, cell isolation, and work attribution
```

The first failed stage/control is recorded; all later stages are `BLOCKED`.
The matrix is conjunctive across all sixteen cells. A positive requires:

- 16/16 cells pass P0--P5;
- 512/512 held-out spans reconstruct with consequence parity;
- 192/192 numbered controls pass;
- 16/16 duplicate replays are identical;
- every exact per-cell accounting value above matches;
- zero forbidden information-flow edges and zero definitive source mutations.

There is no majority, aggregate, or confidence-threshold pass.

## Authority and program transition

If and only if the complete matrix passes, the result authorizes a separate
outcome audit and authoritative handoff declaring:

```text
M3 AUTHORITATIVE

M2
  learned correspondence
  + learned boundary roles
  + learned causal direction

M3
  + learned event organization
```

The same handoff must formally close the **bootstrapping de-supply phase as the
main program** and open the **capability scaling phase**. This is a programmatic
threshold supported by the cumulative chain:

```text
anonymous occurrences
-> correspondence
-> functional roles
-> causal direction
-> event organization
```

The authorized program split after a positive is frozen as:

```text
MAIN CAPABILITY-SCALING PROGRAM
  scaling
  language
  generativity
  stochastic affordances / when randomness is useful
  tool use
  longer-horizon learning
  compute / memory scaling

PARALLEL MINIMALITY PROGRAM
  request/start
  finish/answer
  lifetime/persistence
  plasticity targeting
  semantic credit
```

Further de-supply continues, but it no longer blocks capability scaling.

If any stage, control, cell, exact accounting value, or audit fails, the result
is **CUMULATIVE NEGATIVE** at the first collapse. M2 remains authoritative,
M3 remains absent, and the program-phase transition is not authorized. No
tuning, rescue, threshold change, mechanism change, second matrix, or rerun may
repair the lane.

## Implementation and execution discipline

- Implement only a separate definitive wrapper and serializer over exact
  parent `8e24a13`; do not edit, reformat, regenerate, or retag any frozen
  development file.
- The wrapper may add evaluator-side matrix enumeration, exact-parent/hash
  audit, reporting, and create-new serialization only. It may not change any
  organism decision, observation, threshold, signature, lifecycle mapping, or
  persistent representation.
- The frozen pre-implementation `results/` tree digest is
  `b6dcf5ae5fd782b47f0121705f8b3406c2e00e60a5ec217772677818343a0848`,
  computed by hashing the sorted list of per-file SHA-256 records. Record and
  verify the same digest before definitive implementation and immediately
  before the one-shot run.
- Preflight may use formatting, strict Clippy, focused tests, and one non-claim
  audit using development base seed `83_000` only. Definitive seeds may not be
  probed, sampled, printed, or partially executed before the one-shot command.
- Run all Rust validation and execution with
  `truelearner/scripts/e2b_persistent.py`, credentials
  `truelearner/.env.e2b`, and dedicated state file
  `/Users/satya/.cache/truelearner/ds3-cumulative-definitive-e2b.json`.
- Reuse and leave the dedicated sandbox running; never kill it.
- Execute the frozen `--definitive` command exactly once, single-threaded, on
  E2B. Once the first definitive cell begins, the outcome is spent.
- An infrastructure failure before P0 authorizes no interpretation and does
  not spend the outcome. Any interruption after the first cell begins freezes
  an incomplete definitive outcome and forbids a rerun.
- Serialize only after all sixteen cells finish. Create, never overwrite:
  `results/ds3_cumulative_event_boundary_definitive.csv` and
  `results/ds3_cumulative_event_boundary_definitive.md`.
- Preserve every pre-existing result byte-for-byte, download the two write-once
  artifacts unchanged, and record their SHA-256 digests.
- The outcome audit and M3/program-transition handoff are interpretation-only
  descendants of the write-once artifacts. They may not alter the artifacts or
  implementation.

No definitive implementation or execution is authorized until this protocol
is committed and tagged. This commit spends no definitive evidence.
