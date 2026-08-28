# Core causal diagnostics

```text
drive firing origins
        |
        v
causal-origin resolution -----> propagated drive origin
                                      |
                                      v
modulatory return arrival ----> return-origin decision
                                      |
                                      v
                           reverse-path decision
```

## Outcome

Extend the existing opt-in physical trace with a reusable, read-only causal diagnostic
chain. One run must expose every drive and modulatory origin, whether a junction
preserved or replaced a causal origin, the exact return-origin admission reason, and
the exact reverse-path consolidation reason. Add a public iterator over only those
diagnostic transitions. Learning, scheduling, admission, consolidation, construction,
cost accounting, checkpoints, outputs, and tracing defaults remain unchanged.

## Authority

- Path: `truelearner/crates/core/src/junction.rs`,
  `truelearner/crates/core/src/outcome.rs`, `truelearner/crates/core/src/trace.rs`, and
  `lessons.md`
- Revision: source revision `b94482267e96baffecc9576ddb6878918d9a4974`;
  junction SHA256
  `6e3d54fef4bfb6cebe4225b9eda08bbc89f474bc554cd7c1b2348ff274992901`;
  outcome SHA256
  `7a4a48915bd44a9ceec23e1452a7641d2f9cb2f26b08385f623c68a5141096b4`;
  trace SHA256
  `f660b2735a32b76670fafbc0b231eeaba47f53bb9b9ed4a680b3bd3ae290836b`;
  lessons SHA256
  `7f557e4b11fbc9f1bf404e333d93cb15378b69f66917ef34dee94690f6131fb5`

## Model

`PhysicalEvent` remains the append-only observation vocabulary. New event variants
describe four composable stages: individual firing provenance, junction-level causal
origin resolution, return-origin evaluation, and reverse-path evaluation. Small public
enums make every resolution and rejection reason total and machine-matchable. Existing
events remain unchanged for downstream compatibility.

`RunResult::physical_diagnostics` is a pure projection from the full trace to those new
events. Core mutation remains the effect boundary: each event is appended only when
physical tracing is already enabled and immediately beside the decision it observes.
No diagnostic value is read by the learner.

A drive incidence maps its incoming origins to either `Preserved` when exactly one
distinct origin exists or `JunctionFallback` when multiple origins are collapsed to
the receiving junction's physical identity. Each modulatory firing retains its return
edge identity. Return admission maps all paths to one typed decision, including stale
or invalid edge state, remembered duplicates, direct/local admission, missing origin,
and non-local origin. A successfully admitted origin then maps to exactly one typed
reverse-path decision, including every existing early-return branch and successful
consolidation.

## Invariants

- Tracing remains opt-in and an untraced run emits no physical or diagnostic events.
- Instrumentation is observational: traced and untraced worlds retain identical
  outputs, work, execution cost, quiescence, learner state, link state, and
  state-bearing checkpoint content apart from the returned trace vector and the
  already-persisted tracing flag.
- Existing `PhysicalEvent` variants and their fields retain their meanings.
- Diagnostic events use actual `Firing`, junction, link, generation, distance, and
  decision state; they do not infer intended motion, anatomy, expected identity, or an
  evaluator label.
- Every non-empty sensorimotor drive incidence emits one resolution, and every input
  and modulatory firing emits one provenance event when tracing is enabled.
- Every attempted return-origin evaluation emits one terminal decision. A
  same-moment duplicate that is skipped before normal admission is still visible.
- Every admitted return sent to reverse consolidation emits one terminal reverse-path
  decision, including every pre-existing early return.
- Event ordering is causal and deterministic, and checkpoint replay produces equal
  runs including diagnostics.
- The representative warm regression suite remains strictly under 10 seconds.

## Scope

- Modify `truelearner/crates/core/src/trace.rs`, `junction.rs`, and `outcome.rs`.
- Modify `truelearner/crates/core/tests/harness_boundary.rs` for public-boundary tests.
- Add this plan and factory candidate/verification receipts.
- Exclude learner physics changes, admission-radius changes, origin-resolution changes,
  return-memory changes, construction changes, existing research experiments,
  existing frozen artifacts, campaigns, program claims, benchmarks, and Academy.

## Development style

TDD. Add public-boundary assertions for multi-origin fallback, typed return rejection,
typed reverse failure/success, diagnostics-only projection, replay equality, and trace
opt-out. Confirm they fail because the vocabulary is absent, then implement the
smallest adjacent trace emissions and rerun the full core suite.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_explain_origin_resolution_and_return_decisions`
  checks per-firing provenance, multi-origin junction fallback, modulatory provenance,
  and typed return admission/rejection.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_explain_reverse_path_failures`
  checks that an admitted origin cannot disappear into a silent reverse-consolidation
  early return.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable`
  checks opt-in behavior, state/output preservation, the diagnostic projection, and
  exact within-configuration checkpoint replay.
- `cargo fmt --manifest-path truelearner/Cargo.toml --all -- --check`,
  `cargo check --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --all-targets`,
  and `cargo clippy --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --all-targets -- -D warnings`
  enforce Rust hygiene.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  preserves the full public core contract.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`.
Its measured warm duration must remain strictly under 10 seconds; cold compilation is
recorded separately when it occurs.

## Controls and evidence

Held-out cases are an external firing with no link, two distinct origins meeting at
one junction, a remembered duplicate, a missing/non-local origin, and an admitted
origin whose reverse path cannot consolidate. Negative controls are tracing disabled,
single-origin preservation, existing admission events, exact checkpoint replay, and
unchanged harness observations. Falsifiers are any behavioral or checkpoint drift,
a missing terminal diagnostic decision, diagnostics influencing learner state, a
diagnostic requiring hand-specific knowledge, exhaustive downstream match breakage,
or a warm suite at or above 10 seconds. Expected evidence is passing public-boundary
tests plus validated candidate and independent verification receipts. No scientific
authority or learner-physics claim is produced.

## Risks and rollback

More events increase trace memory only for callers who explicitly enable tracing.
Typed public variants enlarge the API surface and can reveal downstream exhaustive
matches at compile time. Tests detect event-order instability and behavior drift.
Rollback removes only the new diagnostic variants, iterator, adjacent emissions,
tests, plan, and receipts; existing physical events and learner behavior remain.

## Open decisions

None.
