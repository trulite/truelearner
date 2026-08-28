# Retain and project the first workstation contact episode

## Outcome

Add one isolated research experiment that captures the ordinary adopted
workstation session for the existing 120-step recording bound, writes the
complete observer-only recording outside version control when requested, and
projects the retained trace into the preregistered contact-contingency
observables.

The result diagnoses the first broken physical arrow. It does not teach a
movement, alter the world, add learner diagnostics, or claim contact-directed
control.

## Authority

- Path: `research/campaigns/workstation-contact-contingency-v1/protocol.toml`
- Revision: protocol SHA-256
  `480d668bf727f3935b9e044ca7c45f78db1f9a06b74c723fb616b8f094c44eed`
  at parent `119e7293de1aca925e07d0b2deedb2d3e2fc62cd`

## Model

The ordinary causal process is unchanged:

`WorldSample -> WorkstationHarness -> body movement -> WorkstationWorld`

The experiment is a causally inert map from the already complete recording:

`WorkstationRecording -> ContactContingencyEvidence`

For each palm or fingertip site, project before/after `HandPoint`, real
keyboard/touchpad x/y incidence, signed gap to the public contact depth, surface
entry, the next recorded contact sample, physical transition return, movement,
device events, work, and quiescence. A contact episode exists only when an
actual off-surface to on-surface physical transition is followed by positive
pressure at the same local site in the next sample.

Expose the world's fixed contact-depth constant from `academy-workstation` so
the observer and world use one value. Do not expose a target, verdict, action,
or semantic device fact to the learner.

## Invariants

- No change under `truelearner/crates/core`, `truelearner/crates/workstation`,
  body state, learner state, opportunity incidence, checkpoint schema, world
  geometry, initial pose, force law, or session step order.
- Recording remains bounded, checksummed, exact-replay verified, and inert.
- The result retains all five site summaries, every physical surface entry,
  first observed contact, isolated versus coactive digit movement, returned
  transitions, device-event count, maximum step work, and natural quiescence.
- A continued contact sample cannot be mistaken for a first causal entry.
- A device event cannot substitute for local pressure.
- Full 120-step execution is ignored in the warm unit suite and run once as
  separate campaign evidence.

## Scope

- Export the fixed contact depth from
  `academy/crates/academy-workstation/src/world.rs` and `src/lib.rs`.
- Add `research/experiments/workstation-contact-contingency`.
- Add no production dependency from Academy or TrueLearner to research.
- Generated `.tlwr`, frames, and video remain under ignored `output/` and are
  not committed.

## Development style

TDD. Add a focused world threshold test and pure trace-projection tests first,
then implement the smallest public constant and observer projection that make
them pass.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation hovering_does_not_press_but_crossing_depth_does`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation recording_is_an_inert_view_of_the_ordinary_session`
- `cargo test --locked --manifest-path research/experiments/workstation-contact-contingency/Cargo.toml --lib`
- `cargo clippy --all-targets --locked --manifest-path research/experiments/workstation-contact-contingency/Cargo.toml -- -D warnings`
- `cargo run --quiet --locked --manifest-path research/experiments/workstation-contact-contingency/Cargo.toml -- --output research/campaigns/workstation-contact-contingency-v1/artifacts/complete-parent-contact.json --recording output/workstation-contact-contingency-v1/recording.tlwr`

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation recording_`

It must complete strictly under 10 seconds. The complete 120-step campaign run
is measured and recorded separately.

## Controls and evidence

- Held-out cases: real complete session, exact restored replay, surface-entry
  without a next sample at the final bound, continued contact after entry, and
  no-contact trace.
- Negative controls: hover, key threshold, maintained touch, inert recording,
  no device-event substitution, semantic firewall, and no learner/core diff.
- Falsifiers are those frozen in the contact-contingency protocol.
- Evidence is the validated plan, focused checks, complete summary JSON,
  noncommitted recording digest, convergence, and independent verification.

## Risks and rollback

- A specialized observer could accidentally redefine contact. Avoid this by
  sharing the exact fixed threshold and matching the world's x/y predicate.
- A last-step entry has no following sample. Retain it as unmatched rather than
  calling it failed sensation.
- The full replay is intentionally expensive. Keep it outside the warm suite.
- Roll back the experiment and constant export; ordinary session behavior and
  the adopted parent remain unchanged.

## Open decisions

None.
