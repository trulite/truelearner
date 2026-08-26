# RS0 recurrent stability characterization protocol v1

Status: frozen before RS0 observer or evaluator edits.

Parent: CE0 immutable-negative result `ed1a6f4`.

## Question

Is CE0's recurrent excitation an unavoidable consequence of durable reciprocal
coupling, or a consequence of the particular existing timing, threshold,
refractory, and topology geometry used by CE0?

RS0 adds no physical law and tests no plasticity. Every probe body is constructed
already learned with fixed coupling and high resistance. It contains no
Modulatory ARROW or Modulatory input, so coupling, resistance, and participation
cannot learn during the probe.

## Causally inert observation ceiling

Persistent recurrence cannot be passed to the existing unbounded
`propagate()` call. RS0 may add one feature-gated mechanics observer:

```text
propagate_with_observation_ceiling(256 scheduled deliveries)
```

It executes the same queue, ordering, local transitions, clock, and PhysicalWork
as `propagate()`, then pauses only if the next scheduled delivery would exceed
the frozen observation ceiling. It returns the ordinary `RunResult`, number of
scheduled deliveries observed, and whether pending activity remains at the
ceiling. It must not discard, rewrite, attenuate, synthesize, or reorder pending
activity. Calling it again continues the same physical history.

For every naturally quiescent family, the bounded observer must agree exactly
with ordinary unbounded propagation. For every ceiling-reaching family, a
second 32-delivery continuation segment must preserve the same observed regime.
Reference and Production mechanics must agree on each ordered physical prefix.

This is experiment instrumentation, not organism physics. The method is
feature-gated as `rs0`; default runtime behavior remains unchanged.

## Frozen body and observation

- ARROW resistance: `1_000_000`, so local forgetting cannot deallocate or
  materially terminate any RS0 probe within the observation horizon;
- initial Drive impulse: exactly the source CELL threshold;
- no external arrival after the initial pulse;
- no plasticity, Modulation, proposal use, or experimental stop event;
- scheduled-delivery ceiling: `256`;
- persistent-continuation ceiling: `32`.

Record:

- natural quiescence versus observation-ceiling reach;
- ordered deliveries, firings, traversals, and complete physical trace;
- total firings and firings per CELL;
- PhysicalWork and scheduled deliveries;
- final physical tick and retained resistance;
- empirical activity class: `dies`, `periodic`, `persistent_nonperiodic`, or
  `growing`;
- empirical period in firings and ticks where periodic;
- second-segment continuation for persistent cases;
- exact same-mechanics replay and Reference/Production equality.

## Frozen geometries

Run two fresh identity roots, all ten absolute clock phases, and both mechanics
for these twenty existing-physics geometries:

1. one-way coupling 2, delay 1, threshold 2;
2. reciprocal coupling 1, delay 1, threshold 2;
3. reciprocal coupling 1, delay 1, threshold 1;
4. reciprocal coupling 2, delay 0, threshold 2, phase 0;
5. reciprocal coupling 2, delay 0, threshold 2, alternating phases;
6. reciprocal coupling 2, delay 1, threshold 2;
7. reciprocal coupling 2, delay 2, threshold 2;
8. reciprocal coupling 2, delay 3, threshold 2;
9. reciprocal coupling 2, delays 0/1, threshold 2;
10. reciprocal coupling 2, delay 1, alternating phases;
11. reciprocal coupling 2, delay 1, threshold 1;
12. reciprocal coupling 2, delay 1, threshold 3;
13. coupling-2 cycle length 3, delay 1, threshold 2;
14. coupling-2 cycle length 4, delay 1, threshold 2;
15. coupling-2 cycle length 8, delay 1, threshold 2;
16. coupling-2 cycle length 3, delay 0, threshold 2;
17. coupling-2 cycle length 4, delay 0, threshold 2;
18. coupling-2 cycle length 8, delay 0, threshold 2;
19. coupling-2 acyclic chain length 8, delay 1, threshold 2;
20. coupling-2 acyclic chain length 8, delay 0, threshold 2.

Relative phase changes queue order only; existing refractory time remains one
physical tick. Delay zero therefore returns within the initiating CELL's
refractory tick, while any positive total cycle delay may return after it.
RS0 characterizes rather than assumes the result.

## Frozen controls and predicates

- one-way coupling 2 must execute once and quiesce;
- reciprocal coupling 1 at threshold 2 must quiesce;
- reciprocal coupling 2 is characterized, not required to be stable;
- positive-delay cycles of lengths 3/4/8 are characterized for generality;
- zero-delay cycles test existing refractory overlap;
- threshold 3 tests subthreshold coupling 2;
- reciprocal coupling 1 at threshold 1 tests whether instability follows the
  efficacy/threshold relation rather than the numerical value 2;
- both acyclic chains must quiesce after exactly eight CELL firings and must
  never be classified as unstable;
- no ARROW may deallocate or change coupling/resistance during any probe;
- absolute clock phase and mechanics representation must not change a family's
  activity classification.

RS0 succeeds as a characterization if every row is serialized, all hard
controls and invariance gates pass, and each persistent family is observed
through both bounded segments without local forgetting terminating it. RS0 is
not required to find a broad stable regime.

## Decision

- **A — broad existing stable regime:** ordinary timing/refractory geometry
  gives robust quiescence beyond only zero-delay or subthreshold special cases;
- **B — strong recurrence generally persistent:** coupling at or above threshold
  remains self-sustaining across ordinary positive delays and cycle lengths;
- **C — razor-thin stability:** stability depends only on a narrow timing choice
  such as exact same-tick refractory overlap.

The classification is reported after the frozen matrix. RS0 then stops. It may
not add depletion, inhibition, normalization, a coupling ceiling, homeostasis,
or any other candidate law.

