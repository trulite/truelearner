# CJ0-F pre-evidence schedule correction protocol v1

Status: **PREREGISTERED MECHANICAL CORRECTION; EVIDENCE UNSPENT**.

After the v1 evaluator was frozen but before any PROBE result command, a
read-only schedule audit identified one deterministic defect in the shared
held-out-reuse helper. The first acquisition advances a physical instance to
tick 2, while the helper expressed the second matched schedule relative to
absolute tick 0. Either frozen law would therefore reject the backward-time
entry before producing evidence.

The uniquely mechanical correction is:

1. add an explicit physical base tick to the common `enter_genuine` evaluator
   helper;
2. add that base to both contributor and trigger ticks;
3. pass base tick 0 for every existing fresh-world call;
4. pass the already serialized current control tick 2 for both CJ-B and CJ-E
   held-out-reuse calls.

No candidate byte, world topology, identity, marginal, impulse, relative
spacing, phase, threshold, coupling, pressure, expected outcome, accounting
definition, row identifier, matrix dimension, or decision rule may change.
The original implementation tag remains immutable. The result/staging paths
must remain absent through the correction, and the corrected evaluator must
repeat all focused validation and be committed/tagged before PROBE.

This correction creates no definitive/authority surface and does not change
the development-only GATE hard stop.
