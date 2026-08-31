# Formal models

This directory contains executable formal models for claims made about frozen
TrueLearner evidence. They are observer-side tools: no theorem, proof result,
or formal label is admitted to the organism.

`closure/` is the first vertical slice. Lean defines causal ancestry, witness
resolution, ambiguity, and exact persistence; its executable checker accepts a
versioned JSON trace from Rust and returns a versioned receipt.
