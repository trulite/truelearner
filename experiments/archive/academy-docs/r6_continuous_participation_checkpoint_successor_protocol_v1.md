# R6 continuous-participation checkpoint successor protocol v1

Status: frozen before successor evaluator implementation.

## Baseline incompatibility

The frozen R6 evaluator calls `quiescent_checkpoint` at tick 10 after a live
ARROW traversed at tick 1. That fixture predates CPC1 continuous participation.
The ARROW's participation is correctly still nonzero at tick 10, so the
checkpoint is not quiescent.

The unchanged R6 evaluator stopped at the same source line and with the same
`NotQuiescent` result on:

- the AH0 candidate `dfe2602` in fresh E2B worker
  `irl3vfcgtooi5vk9lxjcxs`;
- the exact SI0 v2 parent `3f889bc` in reusable E2B worker
  `ifk44bxtlfjlci644r63m`.

All six partition worlds and all nontrivial partition comparisons execute
before that stale checkpoint assertion. No partition mismatch was reported.

## Authorized successor

Copy the R6 evaluator unchanged except for the quiescent-checkpoint control:

- settle the already-completed body to tick 1024 before requesting the
  quiescent checkpoint;
- keep graph, arrivals, partitions, comparison fields, live-checkpoint control,
  zero added arena latency, and all predicates unchanged.

Tick 1024 is the already characterized CPC1 relaxation endpoint at which the
continuous participation state has returned to baseline. It is evaluator
settling time, not a changed organism input or a supplied causal deadline.

The successor must pass on both the exact parent and AH0 candidate. It does not
change R6's partition-invariance claim and does not modify runtime physics.
