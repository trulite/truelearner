# Formal models

This directory contains executable formal models for claims made about frozen
TrueLearner evidence. They are observer-side tools: no theorem, proof result,
or formal label is admitted to the organism.

`closure/` is the first vertical slice. Lean defines causal ancestry, witness
resolution, ambiguity, and exact persistence; its executable checker accepts a
versioned JSON trace from Rust and returns a versioned receipt.

`verus/` documents the executable-Rust proof boundary. Verus checks the same
return-transition functions called by the body; it does not introduce a second
learner model or feed proof results back into execution.
