# FD1 consequence consolidation implementation audit v1

Status: frozen before physical evidence.

Parent: `fd0-phase-free-local-forgetting-ready-v1` (`d0f9035`).

Protocol: `fd1-consequence-consolidation-protocol-v1` (`7801e5c`).

## Candidate surface

FD1 changes one causal statement in
`truelearner/crates/core/src/lib.rs::apply_modulatory_return`:

```text
before = resistance
resistance += unchanged qualified-Modulation gain
if resistance changed:
    local decay load = 0
```

The reset is nested after the existing gain calculation and is guarded by an
actual durable resistance change. The candidate does not change the gain,
participation impulse, relaxation, local decay period, scheduling, pressure
compatibility fields, proposal law, or transmission law.

Runtime candidate commit: `b145f97`.

Core source SHA-256:
`e7b9d60ce0330d10692b13fe85967e189d734a00177edef98018f9b4499a09ed`.

## Frozen evaluator

The focused evaluator is
`experiments/arms/fd1-consequence-consolidation/src/main.rs`.

It serializes the seven preregistered families C0-C6 across two fresh identity
roots, creation phases 0 through 9, Reference and Production mechanics, and an
exact same-mechanics replay. It compares ordered physical history, candidate
state, work, clock, durable body hash, quiescence, and family-specific
consolidation/death observations.

Evaluator SHA-256:
`010988d9e0f19943a22ba7a4c1813c08d18b5596f8f74fe5ef16babaf60f227d`.

Protocol SHA-256:
`244548ecf8b852949c8b8f0fdeabac379404b2f7c65815229ea82d700192792f`.

## Static boundary

- Traversal continues to change transient participation, not resistance or
  local decay load.
- FD0 local decay reads elapsed time, resistance, and local decay load; it does
  not read participation, plastic support, Modulation, capacity, or global
  pressure phase.
- Wrong-path and zero-participation Modulation compute zero resistance gain and
  therefore cannot reset local decay.
- A dead generation is not resolved by the candidate and cannot be resurrected.
- No scarcity, global sweep, ARC fixture, task horizon, learned class, or event
  reordering was introduced.

## Remote validation before evidence

Reusable E2B worker: `idnc9zn44jihlquq89nvl`.

At committed source `71c0593`, the following targeted, no-world checks passed:

- core workspace rustfmt check;
- evaluator rustfmt check;
- release `cargo check` for the FD1 evaluator;
- release Clippy for the FD1 evaluator with `-D warnings`.

No FD1 physical world, FD0 replay, ARC case, authority matrix, or local Rust
command ran before this freeze.

## Evidence boundary

The next execution may run the focused FD1 matrix once and then rerun the
unchanged FD0 evaluator once. Any focused failure, mechanics mismatch, replay
mismatch, quiescence failure, static violation, or FD0 artifact-hash mismatch
is an immutable FD1 negative. No repair or rerun is authorized inside that
evidence event.
