# PX3-R4 return-window separability result audit v1

Status: **FROZEN R4-C UNINTERPRETABLE; TIMING OVERLAP OBSERVED; PX3 AUTHORITY REMAINS NEGATIVE**.

## Frozen execution

- exact implementation tag:
  `px3-r4-return-window-separability-implementation-v1`;
- exact implementation commit:
  `ff03319461b0030ea830d953081370dd2a9dc8d4`;
- E2B sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- sole evidence marker:
  `PX3_R4_RETURN_WINDOW_SEPARABILITY_EVIDENCE_SPENT`;
- CSV SHA-256:
  `81d3296ddda223486c3e3d00b01e590cc18889e5fffe85e59e1da825f143b82e`;
- report SHA-256:
  `bbf02161d4de8f5f64fec16af545e4758f8c8115a2b03c01e3a28cc430f8e25e`.

The command executed exactly once in E2B. The downloaded artifacts are the
unaltered atomic outputs of that run. No rescue or regeneration was executed.

## Formal outcome

The preregistered result is **R4-C UNINTERPRETABLE**:

- `26/28` rows passed;
- `136/140` independent validity clauses passed;
- exact replay was `28/28`;
- natural quiescence was `28/28`;
- the two failed rows were recurrence offset 1 in the normal and mirrored
  insertion strata;
- each failed only `V1` and `V2`; `V0`, `V3`, `V4`, replay and quiescence
  remained true.

The protocol conjunctively required all 28 rows and all 140 validity clauses,
so the result may not be relabeled R4-B after execution.

## Direct timing measurements

The unchanged physical geometry measured:

```text
first P traversal                         tick 1
lawful R3 M firing                        tick 3
lawful M->P echo arrival                  tick 4
renewed O->P arrival, offset 1..6         ticks 2..7
renewed O->P arrival, offset 3            tick 4
```

Both insertion strata reproduced these timings exactly. Both collision rows
also serialized the preregistered tick-4 source sequence:

```text
renewed O->P unit arrives                 P does not fire
lawful M->P unit arrives                  P fires
uniform external background arrives      P remains refractory
```

Thus the artifacts directly contain a lawful return arrival and a renewed
upstream arrival at the same candidate source on the same tick. This is an
observed timing fact, but the formal R4 classification remains C because of
the separate offset-1 validity failure.

## Unexpected offset-1 collapse

No global return was scheduled in recurrence rows. At offset 1, however,
adjacent episodes produced this physical sequence:

```text
episode 1 P trace -> M at tick 3
episode 1 X trace -> M at tick 3            M state = 2
one tick of ordinary decay                  M state = 1
episode 2 P trace -> M at tick 4            M state = 2
episode 2 X trace -> M at tick 4            M state = 3 -> M fires
M unit echo reaches P at tick 5
```

The attribution cell therefore fired once without the required global-return
participant. Its unit echo did not refire P, but it did lawfully reach the
still-eligible source and raised the candidate from resistance 4 to 7. That is
why the two offset-1 rows failed the frozen expectations that recurrence-only
worlds contain no attribution and no echo.

This is not a memory-management failure. Every row naturally quiesced, exact
replay held, and persistent storage remained bounded (`5792` bytes with one
historical candidate; `5856` bytes when expiry caused a second proposal).

## Scientific boundary

R4 cannot certify temporal separability. Its direct measurements instead show
that the lawful echo window and renewed-input window overlap, including an
exact tick-4 collision. It also reveals a narrower second continuous-time
alias: closely adjacent episode traces can accumulate at M and manufacture an
attribution firing without the third return participant being simultaneous.

No PX0 or PX3 law changed. The failed PX3 definitive authority remains the
governing result, and PX3 authority is still absent.
