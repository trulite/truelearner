# Experiments and authority archive

Everything below this directory is research or evidence, not production
runtime code.

`archive/pxc-authority/` preserves the complete repository layout at PX-C
authority, except that the accepted physical runtime was promoted to
`truelearner/crates/core/` without changing its Rust bytes during relocation.

New evaluators and redesign diagnostics belong under `experiments/`; they may
depend on `truelearner/`, but production code must never depend on them.
