# DS6 cumulative learned-lifetime GATE protocol

Protocol identifier: `ds6-cumulative-lifetime-gate-v1`

Status: **PREREGISTERED BEFORE GATE IMPLEMENTATION OR EVIDENCE**.

This is a development-readiness GATE. It cannot create M4 or authorize a
definitive DS6 execution.

Frozen candidate: the byte-identical scalar lifecycle selected at PROBE and
used unchanged by the positive matched-history diagnostic:

```text
one anonymous signature -> i32 strength map
new recurrence                    strength = 1
existing recurrence/use           strength += 2
each fourth completed event       all strengths -= 1
strength <= 0                     allocation physically disappears
```

No mechanism field, constant, input, update order, pressure clock, or
deallocation rule may change.

## Frozen question

> Across varied histories and loads, does recurrence/use continuously increase
> how much ordinary physical pressure a structure withstands, while continued
> disuse continuously spends that resistance, without any supplied lifetime
> meaning or class and without breaking cumulative M3 behavior?

## Development matrix

Six fresh seeds: `111_000` through `116_000`.

Axes in every seed:

- recurrence counts: `1, 2, 4, 8, 16`;
- subsequent pressure updates: `0, 2, 4, 8, 12, 16`;
- acquisition order: ascending, descending, round-robin, and deterministic
  bursts;
- concurrent one-off load: `8, 32, 128` anonymous signatures;
- surface rendering: fresh occurrences, shapes, local times, consequences,
  allocation order, and reversed presentation;
- contradiction history: none, isolated contradiction, sustained changed
  continuation, and later return of the old lawful signature.

Every pressure update must be caused by ordinary completed M3-compatible
activity. No rest, cleanup, episode boundary, or direct pressure call may enter
the organism path.

## Conjunctive gates

1. **Frozen source and information flow.** Exact M0--M3 ancestry and scalar
   mechanism hashes pass. Lifecycle persistent state contains only anonymous
   signature and scalar strength. No temporary/permanent/TTL/expiry class,
   evaluator delete, retention oracle, future-use signal, semantic correctness,
   request/start, or finish/output value reaches it.
2. **Recurrence ordering.** At every matched non-saturated pressure point,
   final strength is nondecreasing with recurrence; the `2 -> 4 -> 8 -> 16`
   sequence is strictly increasing wherever both compared records remain.
3. **Pressure ordering.** At every matched recurrence count, final strength is
   nonincreasing with pressure and strictly decreases until zero.
4. **Dynamic lifetime.** Measured subsequent pressure updates until physical
   deallocation are nondecreasing for recurrence `1,2,4,8,16`, with strict
   increases for `2 -> 4 -> 8 -> 16`.
5. **Crossed tradeoff.** All recurrence/pressure intersections equal the
   frozen scalar ledger. At least three distinct high-use/long-gap versus
   low-use/short-gap pairs land on different sides of the survival boundary;
   no independent mode or boolean retain state appears.
6. **Interleaving invariance.** Histories with identical per-record recurrence
   and subsequent pressure ledgers give identical strength/survival regardless
   of other records' acquisition order or surface allocation.
7. **Load behavior.** Increasing one-off load increases counted work and raw
   transient allocation but does not change matched target trajectories; every
   unsupported one-off physically disappears.
8. **Gap reuse.** A surviving record reused after gaps `2,4,8` gains exactly
   `+2` through the same recurrence path. A physically removed record
   reacquires from strength `1` without a reinstatement mode.
9. **Contradiction/history.** Changed causal signatures cannot execute stale
   learned paths. Non-use under the changed regime spends old strength; if the
   old lawful signature returns while still allocated it reuses and strengthens,
   otherwise it reacquires ordinarily. No contradiction-specific delete exists.
10. **Cumulative M3 preservation.** Every allocated lawful signature returns
    exact M3 spans, ordinary consequences, learned-use accounting, and natural
    quiescence on fresh renderings; removed signatures fall back to generic M3
    reconstruction and may relearn.
11. **Economy and controls.** Keep-all preserves excess allocations;
    no-pressure over-retains; shuffled recurrence fails to build matched
    resistance; persistent bytes equal physical allocations; occurrence-local
    workspaces return to zero.
12. **Determinism/artifact.** Exact duplicate is byte-identical. The one GATE
    command atomically creates its complete development artifact before exit.

All six seeds, every matrix cell, and every control pass conjunctively. No
aggregate percentage, best-seed selection, or post-hoc tolerance is allowed.

## Outcome discipline

PASS establishes `DS6-CUMULATIVE DEVELOPMENT READY` and permits a separately
preregistered single definitive DS6 matrix. It does not create M4.

FAIL is frozen at its first physical collapse. Only a missing connection
already present in M3 or the byte-frozen scalar mechanism may be repaired and
retried under a separately named protocol. Changing scalar physics, adding a
lifetime representation, or using evaluator future usefulness is forbidden.

Rust formatting, compilation, static audits, and non-seed unit tests occur in
the dedicated E2B development sandbox. The GATE source, runner, and schema must
be committed and tagged before its single execution. Seeds used here are
forever excluded from a definitive matrix.

