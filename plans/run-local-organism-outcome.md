# Run-local organism outcome

```text
link outcome --blocked local choice--> held in this run
     held outcome --first executable local choice--> consumed
```

## Outcome

Project an existing organism link consequence through blocked moments of the
same naturally quiescent run, make it eligible at the first executable local
choice, and consume it exactly once. Test the retained tick-424 to tick-429
reversal without persistent memory or a raw-tick extension.

## Authority

- Path: `language.md`; `lessons.md` lessons 43, 45, 49, 50, 53, and 62;
  retained palm-depth diagnostics at ticks 424, 426, and 429
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`

## Model

When an organism candidate is blocked, its live completing links may already
carry a real consequence. Record those exact link generations and ticks in
`RunState`. A later executable candidate in the same local group may project
only matching live witnesses into its existing held-consequence field. The
ordinary completed-cycle resolver then selects the unique latest outcome.

At that first executable group choice, consume every matching run-local witness
in the group, regardless of the final winner. The list is transient and is
dropped when the run finishes. Learner-held construction outcomes remain
unchanged.

## Invariants

- Only a live completing link's existing `last_consequence_tick` is held.
- Link ID, generation, and consequence tick must all still match.
- Only organism-owned, unambiguous blocked candidates can create a run-local
  hold.
- The hold survives only within one `RunState` and is never checkpointed.
- The first executable local choice consumes every participating hold once.
- Samples cannot invent outcomes, and stale/dead/reused links cannot match.
- No drive, strength, opportunity, return, consequence, path, or learner memory
  is added or changed.
- Production, replay, natural quiet, costs, diagnostic purity, and evaluator
  isolation remain unchanged.

## Scope

- `truelearner/crates/core/src/core.rs`, `physics.rs`, `choose.rs`, and trace
  diagnostics
- focused core lifecycle tests
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes body/checkpoint state, new protocol authority, geometry, target
  knowledge, strength changes, and persistent held paths.

## Development style

TDD: prove hold, generation match, first-choice consumption, and run-end drop in
a core fixture. Then require the unchanged workstation prefix through tick 424
and `0 → 16 → 32` at tick 429. Resume contact only after both pass.

## Focused tests

- A core unit fixture proves exact witness projection and one-use consumption.
- A core run fixture proves a held organism outcome does not cross a second
  `send_physical` call.
- The retained workstation reversal fixture checks tick 429 and exact replay.
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
Negative controls are learner-owned and ambiguous blocked candidates, dead or
reused links, mismatched generations/ticks, a second run, ordinary samples, the
two frozen organism-transition stalls, GenericOnly production, replay, quiet,
and the semantic firewall. Falsifiers are any pre-tick-429 trajectory change,
failure of `0 → 16 → 32`, reuse after the first executable choice or next run,
production change, or a warm suite at or above 10 seconds. Expected evidence is
the lifecycle fixture, retained reversal, stopped contact result, and receipts.

## Risks and rollback

Holding every blocked outcome could join unrelated local groups. Matching exact
completing links at projection and consumption prevents that. Rollback removes
the `RunState` field and its pure helpers; no checkpoint or learner memory needs
migration.

## Open decisions

None.
