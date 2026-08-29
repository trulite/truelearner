# Workstation Visible-Key Intention

## Outcome

Expose one ordinary ANSI keyboard key as visibly illuminated, preserve that
presentation through exact replay, and compare two cue locations under the
unchanged strongest learner. The experiment must identify the first failed
map in:

```text
illuminated key -> retinal difference -> motor difference -> target contact
```

This is development evidence only. It does not claim intentional control.

## Model

- `WorkstationPresentation` is evaluator-owned external world state.
- `illuminated_key: Option<KeyId>` changes only rendered photons.
- `WorldSample` remains the organism boundary and gains no target, key,
  coordinate, direction, score, or evaluator field.
- The learner protocol and topology remain unchanged.
- The paired diagnostic uses the same seed and learner configuration with two
  different illuminated real keys.

The square commutes only if changing the physical cue can eventually change
the selected physical action and contact:

```text
presentation A ----render/sense----> retinal history A
      |                                      |
 change key                            learner physics
      |                                      |
presentation B ----render/sense----> retinal history B
                                             |
                                      motor/contact history
```

## Invariants

1. Default presentation produces exactly the existing uncued rendering and
   behavior.
2. A valid illuminated key is a real key in `WorldGeometry`; invalid IDs fail
   closed.
3. Presentation is included in world/session fingerprints and checkpoints so
   restore reproduces the exact next step.
4. The serialized organism sample contains no evaluator vocabulary.
5. Cue variants differ only in external presentation; seed, learner physics,
   opportunity incidence, and transition law are identical.
6. No learner code may inspect `KeyId`, geometry, target coordinates, or the
   evaluator verdict.
7. The diagnostic reports first full-image divergence, first sampled-retina
   divergence, first body/motor divergence, and target/off-target key events.

## Files

- `academy/crates/academy-workstation/src/world.rs`: presentation state,
  validation, rendering, fingerprinting, and reconstruction.
- `academy/crates/academy-workstation/src/render.rs`: illuminate the selected
  real key.
- `academy/crates/academy-workstation/src/checkpoint.rs`: replay-safe external
  presentation.
- `academy/crates/academy-workstation/src/session.rs`: explicit constructors
  using a presentation.
- `academy/crates/academy-workstation/src/lib.rs`: public Academy boundary
  type.
- `academy/crates/academy-workstation/tests/workstation_world.rs`: rendering,
  firewall, default equality, invalid-ID, and replay checks.
- `truelearner/crates/workstation/src/harness.rs`: expose a read-only research
  retinal projection helper and, if the first square fails there, replace the
  sparse 12-point eye with a regular low-resolution field around each gaze.
- `research/experiments/workstation-return-bearing-opportunity-composition/`:
  paired retained-trace diagnostic and focused test/binary.
- `lessons.md`: record what was observed and what solved it only after the run.

## Verification

1. `cargo fmt --all -- --check` in the affected workspaces.
2. Focused Academy workstation tests with `research` enabled.
3. Focused paired-cue diagnostic/test.
4. Existing first-choice lifetime hand test.
5. Production/research equality and organism-sample firewall tests.
6. `cargo check` and `cargo clippy -- -D warnings` for affected crates.
7. Validate the retained trace by rerunning the same pair and comparing its
   digest; do not promote an intentional-control claim from this development
   run.
