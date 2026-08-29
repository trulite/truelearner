# Workstation proprioceptor delta return

```text
receptor product before --physical movement--> receptor product after
          |                                      |
          +---------- changed active bins -------+
```

## Outcome

Add one research-only workstation arm that returns exactly the active
proprioceptive receptor bins changed by a body movement. It must preserve the
first center-to-signed position change while excluding unchanged static position
at later returns. This tests sustained motion and boundary release without held
paths, desired direction, target knowledge, or production promotion.

## Authority

- Path: `language.md`; `lessons.md` lessons 35, 41, and 53; frozen schemas `workstation-intermediate-transition-contact-trace/v1` and `workstation-effect-receptor-contact-trace/v1`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`

## Model

An axis's proprioception is a product of receptor values. Current position and
velocity reconstruct the pre-movement position. The candidate maps before and
after values to `(offset, bin)` identities and marks a currently active receptor
as a transition exactly when that identity was inactive or different before.
Velocity and effort are effect-born for each action; a newly reached active limit
is also changed. The same pure classifier is used for next-step return and the
immediate sequential resample.

Receptor identity and incidence are separate factors: every proprioceptive input
keeps its stable receptor physical ID, while only the delta-selected inputs carry
`Transition`; the others carry `Sample`.

## Invariants

- Transition incidence and stable receptor origins apply only to changed active
  receptor identities, while unchanged proprioceptors retain the same stable
  origin with sample incidence.
- Unchanged position/center/limit bins and every external/opportunity input stay
  samples.
- No absent receptor is invented as a positive input.
- Output choice, opportunity alignment, tick-order integration, replay, quiet,
  work bounds, and evaluator isolation stay unchanged.
- Whole-axis and effect-only arms remain negative controls; production remains
  byte- and behavior-identical.

## Scope

- `truelearner/crates/workstation/src/harness.rs`
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes core learner changes, new checkpoint state, production promotion,
  device-world changes, and contact authority.

## Development style

TDD: prove center-to-signed inclusion and unchanged-bin exclusion in a pure unit
fixture, preserve two-step forward continuation, then stop the bounded body at
the first two-step boundary release.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation proprioceptor_delta_selects_only_changed_active_bins` proves the delta map.
- `cargo test --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib proprioceptor_delta_preserves_forward_and_boundary_arrows` proves forward continuation, boundary release, and replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation` preserves production behavior and replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable` preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields` preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its budget is strictly under 10 seconds; the early-stopped boundary fixture is
recorded separately.

## Controls and evidence

The held-out case is real keyboard/touchpad contact after release. Negative control
cases are whole-axis upper chatter, effect-only near-origin oscillation,
unchanged receptor bins, GenericOnly production, replay, quiet, and the semantic
firewall. Falsifiers are loss of the initial signed-position transition, return
of unchanged upper position, failure of two-step release, production change, or
a warm suite not under 10 seconds. Evidence is the focused witness plus factory
candidate and verification receipts.

## Risks and rollback

The model cannot emit an absent-channel transition; a later need for explicit
off-events would require a typed complementary receptor, not hidden state. The
research enum and pure classifier can be removed without checkpoint migration.

## Open decisions

None.
