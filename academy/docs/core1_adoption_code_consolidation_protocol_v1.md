# CORE1 adoption code-consolidation protocol v1

Preregistered from commit `3cc6aca98fbfc147bf0ea9d08efd723c55b55e01`
before changing adoption code or running post-consolidation evidence.

This is an adoption gate, not a mechanism experiment and not an ARC run.

## Candidate laws

The ordinary CORE1 execution path must express four physical laws without
evaluator-controlled enablement:

1. A firing input and a local opening form a complete temporary path in the
   same event.
2. Opposite paths are choices, and an output cell holds their signals until
   the current wave finishes.
3. Using a path opens a physical return path; its later outcome closes it.
4. A successful outcome strengthens both used links so a later input can reuse
   the path.

The existing PQLC update rule is frozen. Links may strengthen after a successful
update, but the update rule itself must not be changed.

## Code gate

- The laws live in `truelearner/crates/core/src/lib.rs` on normal CORE1 paths.
- No adopted law depends on candidate booleans, evaluator hooks, seed logic,
  benchmark names, ARC concepts, or Academy-specific behavior.
- Rejected and duplicate candidate implementations are removed or isolated
  from production execution.
- Academy performs endpoint/body wiring only; it does not enable candidate
  physics around an observation.
- Comments describe physical rules, not experiment history.
- Removing experiment-arm calls must not change behavior.

Pre-edit hashes:

- CORE1 implementation: `778782ea97b3dd0c65e5a55a72e0fec8e97329c814e742f88991fdf3c8cb949b`
- Academy adapter: `9ee285a9055c9db712747e50436b65e9dd18a36980f55162b9c2b30db36bf420`

## Regression gate

Run, in order:

1. compile;
2. clippy;
3. focused CORE1 law tests;
4. delayed outcome return and cleanup;
5. learned action from ordinary later input;
6. exact Reference/Production replay;
7. natural quiescence;
8. old Academy contracts;
9. unchanged E14 plus an ordinary autonomous revisit.

The final two behavioral fixtures must call no experimental enable method.
The unchanged E14 update count remains unchanged and is expected to retain its
historical stale-contract failure if both physical halves correctly update.

Every old-contract failure is classified as exactly one of:

- real regression;
- stale expectation;
- unsupported boundary.

No ARC task evidence may run until this gate is frozen.
