# RS1 inhibitory topology sufficiency protocol v1

Status: frozen after the static eligibility audit and before evaluator edits or
execution.

Parent: RS0 characterization result `d1384e8`.

## Static eligibility

The accepted substrate already supports ordinary inhibitory topology without a
new physical type or law:

- `ArrowSpec.coupling` and queued `Spike.impulse` are signed `i32`;
- ordinary Drive delivery applies `CELL.state += spike.impulse` with saturating
  integer representation;
- negative CELL state relaxes toward zero under the existing CELL decay rule;
- firing still requires state at or above the existing positive threshold.

RS1 therefore proceeds. It adds no Inhibitory mode, inhibitory state, depletion,
fatigue, adaptation, normalization, or learning change.

## Question

Can existing CELL/ARROW/SPIKE physics express stable executable recurrent
activity through ordinary local negative-Drive topology?

All bodies are frozen before the probe. There is no Modulatory topology or
input, no coupling/resistance learning, and no CE0 feature. RS1 uses the
feature-gated, causally inert RS0 observation ceiling only.

## Frozen inhibitory geometry

For every excitatory CELL `X` selected for local feedback, add:

```text
X  -- ordinary Drive +1, delay 0 -->  I_x (threshold 1)
I_x -- ordinary Drive -H, delay 0 --> X
```

`I_x` is an ordinary CELL. Both ARROWs use `SourceFires`, ordinary Drive, and
resistance `1_000_000`. The main sufficiency family uses `H=16`, chosen before
execution to remain physically present after the longest frozen cycle return
of eight ticks. A separate frozen sweep uses `H=1,2,3,4,8,16` on the same
two-CELL reciprocal geometry. No result may select a new H after observation.

Excitatory ARROW resistance is also `1_000_000`; local forgetting cannot
terminate the observation. The first observer segment is 256 scheduled
deliveries and a still-active body continues for 32 more.

## Frozen matrix

Use two fresh identity roots, all ten absolute clock phases, ReferencePhysics,
ProductionBody, and exact same-mechanics replay across these 22 families:

1. reciprocal coupling 2, threshold 2, delays 1/1, no inhibition;
2. the same loop with local H16 feedback;
3. inhibited one-way chain length 8, delay 1;
4. inhibited one-way chain length 8, delay 0;
5. reciprocal loop with H16 feedback delivered to disconnected ordinary CELLs;
6. reciprocal loop whose X->I ARROWs exist but cannot fire threshold-2 I CELLs;
7. inhibited reciprocal loop, excitatory delays 0/1;
8. inhibited reciprocal loop, delays 1/1;
9. inhibited reciprocal loop, delays 2/2;
10. inhibited reciprocal loop, delays 3/3;
11. inhibited cycle length 3, delay 1;
12. inhibited cycle length 4, delay 1;
13. inhibited cycle length 8, delay 1;
14. inhibited reciprocal coupling 1 at threshold 1;
15. inhibited reciprocal coupling 3 at threshold 3;
16. uninhibited reciprocal coupling 2 at threshold 3;
17. reciprocal coupling 2/threshold 2 with H1;
18. the same with H2;
19. the same with H3;
20. the same with H4;
21. the same with H8;
22. two simultaneous reciprocal loops, only one carrying local H16 feedback.

The main H16 reciprocal geometry also uses alternating ordinary ARROW phases
across roots/absolute phases; phase remains an existing physical ordering
property, never an inhibition label.

## Frozen observations

Record for each physical neighborhood:

- ordered Drive deliveries including signed impulse;
- CELL firing identities and ticks;
- excitatory and negative-Drive traversal counts;
- natural queue quiescence or both observation ceilings reached;
- activity class and empirical period;
- PhysicalWork, final clock, ARROW coupling/resistance/live state;
- canonical durable body and diagnostic live-checkpoint hashes;
- unbounded equality for every settling body;
- exact replay and Reference/Production equality.

## Hard controls

- uninhibited executable recurrence must reproduce RS0 periodicity;
- H16 local feedback must settle the reciprocal loop;
- both inhibited chains must deliver the intended first traversal through all
  eight excitatory CELLs and then settle;
- disconnected and untraversed inhibitory topology must not stabilize the
  target loop;
- H16 feedback must settle cycles of lengths 2/3/4/8 and excitatory delays
  0/1, 1/1, 2/2, and 3/3;
- H16 must settle executable coupling/threshold pairs 1/1, 2/2, and 3/3;
- uninhibited coupling 2 at threshold 3 must remain an ordinary subthreshold
  quiescent control;
- the strength sweep is characterization: its class may vary with H but must be
  invariant across identity, absolute phase, mechanics, and replay;
- in the simultaneous-loop family, the inhibited neighborhood must settle after
  its intended first cycle while the physically separate uninhibited loop
  remains periodic;
- no ARROW may deallocate or change coupling; Modulation, plasticity updates,
  QLP, and structural proposals must remain zero.

## Decision

- **RS1 positive:** every hard control passes and ordinary H16 inhibitory
  topology settles all frozen executable recurrent geometries without blocking
  the acyclic first traversal. Do not add transmission exhaustion; next ask
  whether suitable stabilizing topology can arise without supply.
- **RS1 negative:** one or more main H16/locality/first-traversal controls fail
  under exact representation-independent execution. A new local
  activity-limiting affordance is then independently justified.

RS1 stops after this discriminator. It does not resume CE0, FD2, ARC, authority,
the oracle, or `arch.md`.

