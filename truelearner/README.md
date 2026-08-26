# TrueLearner production workspace

`truelearner/` is the only production code surface.

The workspace contains one production package:

- `truelearner-core`
  - junction/link state and the transitions in `algo.md`;
  - private resident execution and stable ID/slot resolution;
  - the harness boundary for inputs, outputs, observations, and checkpoints.

The pre-release runtime executes only from explicit mutable RAM. Academy and
tests cannot access the body or resident arena. They send inputs through the
harness, read owned junction/link observations, and may save or restore an
opaque checkpoint. There is no public arena or body persistence format.

Physical Body V1 authority passed `16/16` fresh roots and `540/540` clauses at
tag `physical-body-v1-authority-positive-v1`.

Production crates must not depend on anything under `experiments/`.
Experimental crates may depend on production crates.
