# Organism-view transition continuation

```text
recent physical transition --belongs to--> candidate ownership view
        |                                      |
        +--------- same current arrow ---------+
```

## Outcome

Extend current-transition recognition so an organism-owned candidate can carry
a recent physical transition from an unowned organism origin. Preserve the
existing learner-owned case and reject ambiguous mixed ownership. Test whether
the reversed palm-depth arrow then survives its second step without adding
memory, strength reset, desired direction, or hand knowledge.

## Authority

- Path: `language.md`; `lessons.md` lessons 30, 35, 36, 41, and 53; retained
  palm-depth reversal diagnostic at ticks 424 and 429
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`

## Model

`CandidateOwnership` is the view of a candidate. A causal-lineage origin belongs
to that view when both are owned by the same learner, or when both are unowned
organism physics. Ambiguous candidates have no single view and cannot carry a
current transition. The existing recent-transition window, completing-path
requirement, uniqueness rule, and consequence ranking remain unchanged.

This is a candidate completion of the existing current-transition law. It is
exercised by research transition incidences; ordinary production samples do not
become transitions.

## Invariants

- Learner-owned recognition remains same-owner only.
- Organism recognition requires `CandidateOwnership::Organism` and an origin
  with no learner owner.
- `CandidateOwnership::Ambiguous`, samples, stale transitions, non-completing
  links, and cross-owner origins remain ineligible.
- No strength is reset or copied and no consequence is invented.
- The current-transition winner must remain unique under the existing rule.
- Replay, natural quiet, cost bounds, diagnostic purity, semantic isolation,
  and production behavior stay unchanged.

## Scope

- `truelearner/crates/core/src/choose.rs`
- focused core tests
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes new memory, protocol promotion, checkpoint changes, world geometry,
  target knowledge, and contact authority.

## Development style

TDD: first prove the pure ownership-view relation for learner, organism, and
ambiguous candidates. Then require the retained lower-depth reversal to move
from 0 to 16 to 32. Only after that square commutes, resume the compact contact
fixture and stop at its next falsifier.

## Focused tests

- A core unit fixture proves same-learner and organism-view transitions while
  rejecting ambiguous, cross-owner, sample, and stale cases.
- `palm_component_carries_the_reversed_depth_arrow_for_two_steps` proves the
  retained first wall.
- The compact palm-component contact fixture runs only after the reversal
  witness passes.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves production behavior and replay under the strict warm budget.
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
Negative controls are ambiguous ownership, cross-owner origin, ordinary sample,
stale transition, non-completing link, the retained old-path dominance trace,
GenericOnly production, replay, natural quiet, and the semantic firewall.
Falsifiers are failure to carry 0 to 16 to 32, admission of any negative
control, loss of the existing learner-owned case, production change, or a warm
suite at or above 10 seconds. Expected evidence is the core relation fixture,
the reversed-depth witness, the stopped contact result, and factory receipts.

## Risks and rollback

An optional learner ID alone conflates organism and ambiguous candidates. The
implementation must therefore use `CandidateOwnership` for recognition and
must not infer organism from an absent owner. The relation change can be
removed without checkpoint migration. Rollback restores the learner-only guard
and the retained workstation reversal remains an explicit negative control; no
stored data or evidence envelope needs migration.

## Open decisions

None.
