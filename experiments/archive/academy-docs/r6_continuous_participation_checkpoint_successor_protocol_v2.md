# R6 continuous-participation checkpoint successor protocol v2

Status: frozen before v2 evaluator implementation.

The quiescent checkpoint fixture must not guess a supplied settling tick.
Beginning at the predecessor's tick 10, advance ordinary physical time one tick
at a time until the existing public `quiescent_checkpoint` contract first
accepts the body. Only `NotQuiescent` may continue the search; any other error
stops. A fixed evaluator safety ceiling may stop the experiment but cannot
alter organism physics.

Schedule the control's future arrival exactly one tick after the observed
quiescent checkpoint. Keep the body, first arrival, partition, comparison,
live-checkpoint control, and zero-latency R6 worlds unchanged.

Required:

- the first legal quiescent tick is deterministic and replayable;
- the exact SI0 v2 parent and AH0 candidate choose the same tick;
- all partition worlds and both checkpoint controls pass identically;
- no runtime source change is permitted.
