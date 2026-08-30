# Sparse physical sensor memory

```text
current value --> sampled body junction --> crossed-threshold links --> rise/fall
                         |
                         `-- retains only its prior value and lifetime
```

## Outcome

Replace the workstation raw sensor memory's repeated per-threshold refresh network
with one sparse, body-native sampled junction. Preserve initial silence, exact rise and
fall outputs, physical lifetime, action origin, replay, natural quiescence, and
the complete workstation ladder while removing most of the 10,413 deliveries
currently added by runtime sensor memory.

## Authority

- Path: `truelearner/crates/core/src/junction.rs`,
  `truelearner/crates/core/src/attachment.rs`,
  `truelearner/crates/embodiment/src/lib.rs`, and
  `truelearner/crates/workstation/src/harness.rs`
- Revision: `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a` plus the current candidate tree

## Model

The objects are an observed scalar band, a physically retained prior value, and
threshold rise/fall spikes. One sampled junction transforms its prior state and
the current observation into a new state plus the coproduct of thresholds
actually crossed. Triggered ordinary links carry only those crossings. The
generic integrating junction and authoritative calibration construction are
unchanged; only continuously sampled workstation raw traces select the sampled
construction.

## Invariants

- The first observation emits no rise or fall; equal values remain quiet.
- Every crossed threshold emits exactly the same directional output as the
  existing trace, including multi-threshold jumps and expiry at the declared
  lifetime.
- Retained memory consists only of attached junction activation and links; no
  workstation or adapter cache remembers the previous sensor value.
- Physical cause, transition incidence, checkpoint replay, natural quiescence,
  and complete-candidate predicates are preserved.
- Focused vision remains sparse and unchanged.

## Scope

- Add an attached sampled-junction law and directional threshold link triggers.
- Add a compact sampled-value physical trace construction and use it for runtime
  raw workstation channels.
- Retain reusable cycle-lens input and delivery attribution.
- Exclude learner-law changes, calibration-trace replacement, evaluator changes,
  threshold removal, and authority promotion.

## Development style

TDD: add compact-trace equivalence, lifetime, replay, and delivery-bound tests,
then implement the smallest component and junction behavior that satisfies them.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment compact_trace -- --nocapture`
  proves directional behavior, expiry, replay, and reduced physical deliveries.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --features research --test workstation_harness runtime_attached_complete_surface_is_inert_then_covers_owned_sensors -- --exact`
  proves runtime attachment and replay remain intact.
- `cargo run --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --bin runtime_attached_physical_cycle_lens -- --one-step`
  records the exact delivery and CPU reduction.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib runtime_attached_complete_candidate_retains_first_broken_arrow`
  preserves the complete ladder and cost boundary.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment && cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --features research --test workstation_harness`.
Its warmed duration must remain strictly under 10 seconds.

## Controls and evidence

Held-out cases are a multi-threshold jump, unchanged high and low observations,
and a sample after physical-memory expiry. The negative control is the unchanged
generic trace and calibration construction. Falsifiers are any changed rise/fall
set, hidden previous-value state outside the body, replay mismatch, non-quiescence,
complete-ladder regression, or a complete one-step delivery count above 7,000.
Expected evidence is the focused test output and cycle-lens comparison; no large
trace or recording is committed.

## Risks and rollback

Incorrect coincidence lifetime could erase evidence too early, and one-hot band
indexing could reverse a threshold. Direction, expiry, replay, and complete-ladder
tests detect these failures. Rollback restores the workstation raw trace builder;
the generic physical trace and calibration authority remain untouched.

## Open decisions

None.
