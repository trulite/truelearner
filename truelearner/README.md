# TrueLearner production workspace

`truelearner/` is the only production code surface.

The workspace may contain multiple mechanically focused crates. The initial
baseline contains one package, `truelearner-core`, with:

- `src/lib.rs`: the accepted physical state and transition rules;
- `src/main.rs`: the production composition root.

Production crates must not depend on anything under `experiments/`.
Experimental crates may depend on production crates.
