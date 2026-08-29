# Organism transition divergence

```text
current physical arrow -----> current executable candidate
          |                                  |
 ordinary choice ---------------------> chosen candidate

repair only when the two arrows differ
```

## Outcome

Prefer a unique executable organism candidate carrying the newest current real
transition only when ordinary strength ranking would choose a different older
arrow. Preserve the ordinary winner when it already agrees, and leave blocked
opportunity and unchanged-boundary release untouched.

## Authority

- Path: `language.md`; `lessons.md` lessons 0a, 0b, 35, 41, 42, 43, and 53;
  retained reversal divergence at ticks 424/429; failed broad and
  executable-wide organism arms
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`

## Model

Each executable candidate records whether a completing path carries a recent
transition from the organism view. Ordinary ranking is computed first. A pure
resolver may replace it only when exactly one organism-current candidate has a
strictly newer consequence than the ordinary winner. If the winner is already
that candidate, the resolver is identity. Learner-owned continuation keeps its
existing priority and behavior.

The organism-current fact is not used by direct opportunity, candidate
promotion, return admission, or unchanged-sample release. This makes the repair
local to the first non-commuting choice square.

## Invariants

- The complete pre-divergence choice sequence remains equal to the parent arm.
- Only an already executable, unambiguous organism candidate can participate.
- The candidate must carry a recent real transition and a strictly newer actual
  consequence than the ordinary winner.
- Identity choices, ties, samples, stale transitions, ambiguous ownership, and
  blocked candidates remain ordinary.
- No drive, opportunity, path, return, consequence, or memory is created.
- Learner-owned continuation, boundary release, replay, quiet, work bounds,
  production, and evaluator isolation remain unchanged.

## Scope

- `truelearner/crates/core/src/choose.rs`
- focused resolver tests and continuation diagnostic
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes strength changes, new state, checkpoint changes, geometry, target
  knowledge, and authority promotion.

## Development style

TDD: prove identity, unique-newer divergence, tie, and stale controls in a pure
resolver. Then compare the retained prefix through tick 424 and require
`0 → 16 → 32` at tick 429. Resume contact only after both pass.

## Focused tests

- A core pure resolver fixture covers identity, unique newer replacement, tie,
  and stale controls.
- A workstation fixture proves the retained prefix and two-step reversed depth
  arrow with `CurrentTransition` as the choice basis.
- The compact contact fixture stops at first pressure or first repeated pose.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves production behavior under the strict warm budget.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable`
  preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its budget is strictly under 10 seconds. Research fixtures are separate and
stop at the first falsifier.

## Controls and evidence

The held-out case is sustained motion to real keyboard/touchpad contact.
Negative controls are ordinary/current identity, equal consequence ties, stale
transitions, ambiguous ownership, blocked candidates, unchanged-boundary
release, the two frozen over-broad stalls, GenericOnly production, replay,
quiet, and the semantic firewall. Falsifiers are any parent-prefix divergence
before the retained wall, failure of `0 → 16 → 32`, intervention on an identity
or negative control, production change, or a warm suite at or above 10 seconds.
Expected evidence is the pure resolver fixture, prefix comparison, reversed
depth witness, stopped contact result, and factory receipts.

## Risks and rollback

The resolver could accidentally use transition freshness without actual
consequence freshness. Both facts are required independently. Rollback removes
the organism divergence resolver and restores the retained old-strength winner;
no checkpoint or evidence migration is required.

## Open decisions

None.
