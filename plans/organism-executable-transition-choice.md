# Organism executable-transition choice

```text
physical opportunity makes candidates executable
                         |
          current organism transition
                         |
                         v
             choose one existing arrow
```

## Outcome

Allow a recent organism-view physical transition to choose among already
executable local candidates, while forbidding it from supplying opportunity,
promoting a blocked candidate, or opening a return. Test whether the newly
reversed palm-depth arrow then beats an older stronger path without freezing
the hand.

## Authority

- Path: `language.md`; `lessons.md` lessons 30, 35, 36, 41, and 53; retained
  old-path dominance at ticks 424/429; failed broad organism-view arm frozen in
  `plans/organism-view-transition-continuation.md`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`

## Model

Candidate ownership defines the view of a causal transition. Same-learner
origins preserve the existing behavior. An unowned transition may mark an
organism candidate as current only after ordinary drive and opportunity have
already made that candidate executable. Ambiguous ownership remains
ineligible. The existing unique-latest current-transition resolver may then
rank the executable candidates.

The organism-view mark must not participate in direct transition opportunity,
blocked-candidate promotion, fresh-opportunity transfer, or return admission.
This separates selection from energy and addresses the frozen `(528, 768, 256)`
counterexample produced by the broader arm.

## Invariants

- Organism currentness is computed only for an already executable candidate.
- It can affect only competition among executable candidates.
- It cannot add drive, opportunity, a consequence, an unanswered return, or a
  path.
- Learner-owned direct opportunity and return-bearing behavior remain
  unchanged.
- Ambiguous, cross-owner, sample, stale, and non-completing inputs remain
  ineligible.
- Replay, natural quiet, work bounds, diagnostic purity, semantic isolation,
  and production behavior stay unchanged.

## Scope

- `truelearner/crates/core/src/choose.rs`
- focused core tests
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes new memory, strength changes, checkpoint changes, world geometry,
  target knowledge, and authority promotion.

## Development style

TDD: prove with a pure resolver fixture that organism currentness cannot promote
a blocked candidate, then require the retained depth sequence `0 → 16 → 32`.
Only if it passes, resume the compact contact fixture and stop at the next
falsifier.

## Focused tests

- A core unit fixture proves organism currentness ranks executable candidates
  but does not satisfy direct-opportunity or promoted-return admission.
- A workstation fixture requires `0 → 16 → 32` at the retained first wall and
  checks that the winner basis is current transition.
- The compact contact fixture runs only after the reversal witness passes.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves production behavior under the strict warm budget.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable`
  preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its budget is strictly under 10 seconds. Research reversal and contact fixtures
are recorded separately and stop at the first falsifier.

## Controls and evidence

The held-out case is sustained motion toward real keyboard/touchpad contact.
Negative controls are the frozen broad-arm stall, blocked organism candidates,
ambiguous ownership, samples, stale transitions, non-completing links,
GenericOnly production, replay, natural quiet, and the semantic firewall.
Falsifiers are failure of `0 → 16 → 32`, any organism transition creating
opportunity or a return, recurrence of the frozen pose, production change, or a
warm suite at or above 10 seconds. Expected evidence is the core gating fixture,
the reversed-depth witness, the stopped contact result, and factory receipts.

## Risks and rollback

The internal candidate currently stores learner ownership separately from its
diagnostic ownership class. The implementation must retain the class long
enough to distinguish organism from ambiguous candidates. Rollback removes the
organism-only executable mark and restores the retained old-path-dominance
negative control without checkpoint migration.

## Open decisions

None.
