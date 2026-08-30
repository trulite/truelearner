# Route Academy's frozen boundary through Workstation

```text
Academy -> truelearner-workstation::boundary -> frozen core Harness
```

## Outcome

Remove every direct `truelearner-core` dependency and import from the Academy
workspace. Re-export the exact frozen boundary types Academy already uses from
`truelearner-workstation`, then change Academy to import those names through
Workstation. Preserve runtime behavior, checkpoints, traces, and evidence
exactly. This cycle changes dependency ownership only; it does not replace or
modify any Harness or learner implementation.

## Authority

- Path: `academy.md`, `arch.md`, the current Academy core and ARC3 adapters,
  and the public `truelearner-workstation` crate boundary.
- Revision: Git `ded2e725622e270ad0d414dc433d1ee965f8145d`; `academy.md` SHA-256
  `9b1116391ba49f09d72e27b4af5dbaff73c667730c300899420b7b58429b9cd0`;
  `arch.md` SHA-256
  `02d837a8dc205aae7b088147226c94aa08783898a653550334718bbdf0cc003f`.

## Model

The runtime object and every transformation remain unchanged. Workstation adds
one explicit `boundary` module that re-exports the concrete checkpoint,
Harness, builder, input, output, physical topology, trace, error, and work types
used by Academy. Academy changes only the path by which those types are named.

The sole transformation introduced by this cycle is a dependency factorization:
an Academy reference to a frozen boundary type factors through Workstation and
resolves to the identical Rust type. Therefore values, failures, serialization,
mutation, physical time, and observation compose identically before and after.

## Invariants

- No file under `truelearner/crates/core` changes.
- `truelearner/crates/body/src/harness.rs` and
  `truelearner/crates/workstation/src/harness.rs` do not change.
- No learner, body, Harness, checkpoint, morphology, input, output, or physical
  behavior changes.
- Academy Cargo manifests and Rust sources contain no `truelearner-core` or
  `truelearner_core` reference.
- Academy-visible types remain the exact original concrete types, not wrappers
  or conversions.
- Expected answers, scores, evaluator state, and capability names remain
  outside the Harness.
- Existing Academy replay, checkpoint, ARC3, and workstation tests remain
  unchanged and pass.
- The representative warm regression remains strictly under 10 seconds.

## Scope

- Add the narrow compatibility re-export module to
  `truelearner/crates/workstation/src/lib.rs` without changing its Harness
  module.
- Replace the Academy workspace dependency and the direct dependencies in
  `academy-core` and `academy-arc3` with `truelearner-workstation`.
- Change imports and fully-qualified checkpoint error references in
  `academy-core` and `academy-arc3`.
- Update the Academy lockfile if Cargo requires it and add candidate and
  verification receipts.
- Exclude all core, body, Harness, Embodiment, Workstation runtime, checkpoint
  format, curriculum, evidence schema, UI, and historical research changes.

## Development style

Use implementation-first because the change is a type-identity-preserving
dependency factorization. Apply the re-export and import substitutions, then
compile and run the unchanged tests as the behavioral oracle.

## Focused tests

- `cargo test --manifest-path academy/Cargo.toml -p academy-core`
  checks the headless Academy boundary, A1 behavior, checkpoints, and replay.
- `cargo test --manifest-path academy/Cargo.toml -p academy-arc3`
  checks ARC3 sensorimotor behavior and physical trace ownership.
- `cargo test --manifest-path academy/Cargo.toml -p academy-body -p academy-workstation`
  checks the existing Workstation-mediated Academy paths.
- `cargo check --manifest-path academy/Cargo.toml --workspace --exclude academy-playground`
  checks every non-UI Academy consumer through the new dependency edge.
- Zero-match `rg` checks for `truelearner-core` in Academy Cargo manifests and
  `truelearner_core` in Academy Rust sources establish the direct-dependency
  boundary.
- `git diff --exit-code -- truelearner/crates/core truelearner/crates/body/src/harness.rs truelearner/crates/workstation/src/harness.rs`
  is compared with the captured pre-change diff so this cycle adds no changes
  to frozen files.

## Development loop

The representative warm regression is
`cargo test --manifest-path academy/Cargo.toml -p academy-core --lib`.
Measure it after one bootstrap run and require elapsed wall time strictly under
10 seconds. Record cold bootstrap separately.

## Controls and evidence

Existing Academy tests are held-out behavioral controls because they retain the
same concrete boundary types and expectations. Negative controls are the
zero-match dependency scans and the frozen-file diff comparison. This rewrite
is falsified by any test difference, a remaining direct core reference, a new
conversion or wrapper, a frozen-file change introduced by this cycle, or a warm
regression at 10 seconds. Expected evidence is validated candidate and
verification receipts; no capability or learner-physics claim is produced.

## Risks and rollback

The main risk is accidentally presenting a wrapper as a behavior-preserving
re-export. Re-export the exact types and add no methods or conversions. Existing
uncommitted changes in frozen files predate this cycle; capture and compare
their diffs rather than assuming a clean worktree. Rollback removes the
Workstation re-export module and restores the two direct manifest dependencies
and import paths.

## Open decisions

None.
