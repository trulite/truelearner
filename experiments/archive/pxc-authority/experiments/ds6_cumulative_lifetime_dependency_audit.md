# DS6 cumulative lifetime dependency audit

Status: **STATIC AUDIT; NO DS6 MECHANISM OR SCIENTIFIC EVIDENCE**.

Parent: authoritative M3, tag `m3-cumulative-event-boundary-authoritative`.
Target: remove supplied temporary/permanent state and supplied lifetime or
persistence classes under the frozen DS6 forbidden-information clauses.

## M3 state inspected

The byte-frozen isolated DS3 core has SHA-256
`a8d8fe060b497c7a6b5f9a5a88b7ed2292dc8a729a8781f599547b6027efc0a0`.
Its persistent region contains two structurally distinct stores:

```text
support: signature -> recurrence count
chunks:  signatures that crossed supplied CONSOLIDATION_SUPPORT = 2
```

Thus M3 already exposes recurrence, learned reuse, invalidation activity,
persistent byte cost, and ordinary downstream execution. But the supplied
threshold and the separate `support`/`chunks` stores preclassify acquisition
state into de facto provisional and consolidated tiers. M3 has no physical
path by which disuse, contradiction, storage pressure, or reuse cost changes
whether a signature remains stored.

## Mechanical dependency scan

| Order | Required physical dependency | Static state | Finding |
|---:|---|---|---|
| 1 | recurrence and use reach persistent state | partial | recurrence increments support and learned chunks are counted as used |
| 2 | one evaluator-blind lifecycle controls retention and erasure | absent | no cumulative DS6 port exists |
| 3 | no temporary/permanent class or structural tier | absent | M3 separates support from chunks using a supplied threshold |
| 4 | retention cost and later reuse are observable | present but unlinked | bytes and learned uses are measured but do not affect persistence |
| 5 | history and local pressure update persistence | partial | recurrence adds; invalidation reopens execution; nothing decays or erases |

The first missing physical edge is therefore:

> M3 activity does not enter one evaluator-blind lifecycle capable of both
> retaining and removing state without a supplied lifetime class.

## Frozen parts-bin interpretation

Existing results constrain, but do not yet select, the lifecycle:

- v17 recurrence consolidation physically removes one-off patterns while
  preserving behavior, but supplies rest timing, recurrence threshold, rewrite,
  and replay validation;
- d2.3 retains historical evidence and locally reopens on contradiction, but
  supplies a violation threshold and does not erase persistent state;
- d2.4 shows that one mutable value/tau trace can be stable and reopen, but is
  a frozen negative for efficient repeated-switch historical reuse;
- d2.6/d2.6m show causal, economically useful reuse of anonymous addressed
  history, while d2.6e is evaluator-side accounting rather than an
  organism-visible lifecycle;
- IR0 invalidation/reopening reuses history but does not learn what to retain;
  M3's own recurrence support learns what repeats but not what should decay.

No result licenses copying a supplied rest event, lifetime threshold, future
reuse oracle, semantic state kind, or accounting classification into DS6.

## Unresolved diagnostic fork

Three non-semantic evidence families remain live:

```text
A  recurrence/use competition
   local recurrence and actual reuse oppose ordinary decay/storage pressure

B  surprise/contradiction timescale
   agreement stabilizes while contradiction restores plasticity or decay

C  dependency/economic pressure
   downstream dependence and realized reuse oppose local carrying/work cost
```

These are diagnostic arms, not approved representations. A frozen PROBE must
expose the same M3-derived state, recurrence, disuse, contradiction, reuse, and
physical pressure to each arm. It may select an arm only from preregistered
physical discriminators: preservation of useful recurrent state, removal of
unused or contradicted state, later reuse behavior, and local work/storage.

If no arm is uniquely supported, or if success requires a new persistent
representation rather than a mechanical reuse of the frozen parts bin, DS6
stops for scientific ambiguity before MICRO.

