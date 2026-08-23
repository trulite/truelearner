# PX3-D2 recursive normalization protocol v1

Status: **PREREGISTERED; EVIDENCE UNSPENT; FULL RECURSION ABSENT**.

Start: frozen D1 result commit `c01bdb42cc1a146284154ab3f8aa801df36f9383`.
Authoritative PX0--PX2 and all spent PX3/D1 artifacts are read-only.

## Question

> When a mature physical AB organization executes, can the authoritative PX1
> normalization motif convert that execution into exactly one ordinary unit
> participation trace X, indistinguishable downstream from a primitive trace?

D2 does not learn AB, attribute return, form candidates or recurse. Mature AB
is supplied uniformly as fixed physical topology so only normalization and
level-blind participation are tested.

## Exact topology

Primitive A/B/C/D use the authoritative PX1 source/outlet/shared-return/trace
motif. Unit A and B traces converge on threshold-two `O_AB`. `O_AB` traverses a
fixed mature organization ARROW of raw coupling `1`, `2` or `4` into an
ordinary X outlet. That outlet uses the same PX1 motif as a primitive:

```text
X outlet execution
  -> one unit local impulse to X trace(threshold 2)
  -> one unit through ordinary X return hub
  -> exactly one X trace firing
```

X trace and primitive C trace feed an ordinary threshold-two downstream
opportunity with identical coupling-one ARROWs. The downstream CELL has no flag
indicating that X came from conjunction.

## Frozen matrix

Seeds `3101, 3109`; normal and mirrored insertion order; load `0`.

1. A alone;
2. B alone;
3. strong A(4) alone;
4. repeated A;
5. AB through mature coupling `1`;
6. AB through mature coupling `2`;
7. AB through mature coupling `4`;
8. AB-derived X timed with primitive C -> downstream X+C fires once;
9. X and C outside the trace-overlap window -> downstream does not fire;
10. primitive D timed with primitive C -> matched primitive+primitive
    downstream opportunity fires once.

Required:

- all singleton/repetition controls produce no AB, X outlet or X trace;
- every mature coupling produces one AB firing, one mature traversal carrying
  its raw coupling, one X outlet firing and exactly one unit X trace firing;
- raw mature amplitude never multiplies X participation;
- X+C and D+C each present two unit traces to the same ordinary downstream API
  and fire once when overlapping;
- gapped X/C does not fire;
- exact replay and natural quiescence hold.

Serialize all source, raw, outlet, trace, mature, X-normalization and downstream
native crossings/impulses plus work, fingerprints and storage. No coupling is
inferred from resistance.

## Verdict

- **D2-A:** mature composite execution becomes one ordinary unit participant
  and passes the downstream equivalence controls.
- **D2-D/E:** success requires a typed composite adapter, emits multiple traces,
  or the complete matrix is materially negative.

A positive D2 does not establish Y/Z learning or full recursion. D1-R2, D2,
MICRO, GATE and authority remain independent. Implementation requires a
separate frozen execution surface, E2B preflight and one write-once run.
