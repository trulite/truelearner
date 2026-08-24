# TrueLearner production/research relocation protocol v1

## Parent

The relocation begins at PX-C continuous-organism authority commit
`ec87c438aa8c52389fd2734667363ef43acaef93`.

## Purpose

Separate the production organism from the research archive without changing
the accepted physical state or transition rules.

## Required layout

```text
truelearner/
    production Rust workspace only
    crates/core/src/lib.rs   accepted physical runtime
    crates/core/src/main.rs  production composition root

experiments/
    all predecessor code
    all evaluators
    all protocols and audits
    all generated evidence and research tooling
```

Production crates must not depend on `experiments/`. Experimental verification
may depend on production crates.

## Frozen invariants

- The relocated `core/src/lib.rs` bytes must equal the PX-C authority runtime.
- No physical field, transition, threshold, schedule, pressure rule, proposal
  rule, boundary rule, or observation rule may change.
- The archived research tree must remain available under one prefix.
- The relocated PX-C development matrix must pass without executing a fresh
  authority matrix.
- Exact replay, natural quiescence, outward-only crossings, and work/memory
  bounds must remain true.
- Formatting, strict Clippy, and targeted tests must pass in E2B.

## Explicitly out of scope

- Promoting this branch to `main`.
- Runtime redesign or optimization.
- Adding a host protocol, framebuffer, distribution, foveation, or persistence.
- Removing accepted fields or observers.
- Changing `arch.md` authority claims.

The successful relocation becomes the clean parent for subsequent redesign.
