# DS6 matched-history lifetime diagnostic protocol

Protocol identifier: `ds6-cumulative-lifetime-matched-history-v1`

Status: **AMENDED BEFORE IMPLEMENTATION OR FOLLOW-UP EVIDENCE**.

This pre-evidence amendment replaces the preliminary recurrence counts with
`1, 2, 4, 8` and adds a crossed recurrence/disuse comparison. No matched-
history learner code or seed has been executed.

This is not a rerun or rescue of
`ds6-cumulative-lifetime-micro-v1`. That matrix remains a valid negative at its
4/4 useful-persistence gate. This separately named diagnostic asks the first
mechanically forced question exposed by that negative:

> With one variable matched, does the unchanged scalar mechanism derive
> physical lifetime from the other variable: recurrence history or elapsed
> ordinary pressure?

## Frozen mechanism

The implementation tagged `ds6-cumulative-lifetime-micro-implementation`
remains exact:

```text
one anonymous signature -> i32 strength map
new recurrence                    strength = 1
existing recurrence/use           strength += 2
each fourth completed event       all strengths -= 1
strength <= 0                     allocation physically disappears
```

No field, constant, input, update order, pressure clock, or deallocation rule
may change. No temporary/permanent/TTL/expiry class, evaluator delete,
retention oracle, future usefulness, request/start, or finish/output value may
enter the organism path.

## Fresh development cells

Seeds `109_000` and `110_000` are disjoint from all prior development and
definitive populations.

### Recurrence-matched-pressure axis

Four independent blank lifecycles receive recurrence counts `1, 2, 4, 8`.
After the last use, each receives exactly four subsequent physical pressure
updates. Ordinary filler events are added only as needed to reach those four
updates; the evaluator may not call pressure directly.

Pass requires final scalar strength and/or survival to be monotonically ordered
by recurrence count. Strict increase is required before saturation, but the
one-recurrence record is permitted to reach zero under the common pressure:

```text
strength(1) <= strength(2) < strength(4) < strength(8)
```

### Disuse-matched-recurrence axis

Four independent blank lifecycles each receive exactly six recurrences. They
then receive respectively `2, 4, 8, 12` subsequent physical pressure updates,
again caused only by ordinary completed activity.

Pass requires monotonically decreasing strength, survival at short and medium
gaps, and physical disappearance at the longest gap:

```text
strength(2) > strength(4) > strength(8) > strength(12) = 0
```

### Crossed history-pressure interaction

Two additional blank lifecycles compare:

```text
high recurrence   8 uses + 8 subsequent pressure updates
low recurrence    2 uses + 2 subsequent pressure updates
```

The update law receives no branch identifying either case. The diagnostic
records both final strengths and survival states. Pass requires the outcome to
equal the algebraic scalar trajectory from the frozen `+2/-1/zero-remove`
law, demonstrating that use and pressure combine in the same state variable.
No direction such as “high must beat low” is imposed post hoc: the frozen
physical ledger decides the comparison.

### Reuse-after-gap

A six-recurrence record receives four physical pressure updates, is observed
again through the ordinary recurrence path, and must increase by exactly `2`
without a special reopening or reinstatement branch.

## Additional conjunctive controls

Both seeds must also show:

1. fresh identity, surface shape, consequence, local time, and reversed
   allocation leave both orderings exact;
2. one-off records disappear and reacquire through the same path;
3. a contradicted causal signature cannot use the stale learned record;
4. no-pressure retains all records and therefore discriminates;
5. persistent bytes track only physically allocated records;
6. duplicate execution is byte-exact and occurrence-local state returns zero;
7. the single command atomically creates its development result artifact.
8. the crossed interaction equals the preregistered scalar ledger in both
   fresh identity/layout renderings.

There is no aggregate threshold. Both axes and every control pass in both
seeds or the diagnostic is negative.

## Interpretation and next step

PASS would establish only the development signature:

> Identical supplied scalar physics yields different physical lifetimes from
> different recurrence and disuse histories, with no lifetime label.

It would make a broader DS6 GATE eligible. GATE must then test the same strict
dynamic ordering across more seeds, pressure schedules, interleavings, and
capacity loads while preserving the frozen cumulative M3 behavior.

FAIL preserves M3 authority and follows the first missing physical edge. The
follow-up may not tune the selected scalar mechanism.
