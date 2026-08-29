# Link outcome first-choice lifetime

```text
real link consequence --available across runs--> first local choice --consume--> history only
```

## Outcome

Add one research-only protocol that keeps an organism-owned consequence on its
existing live link until that link first participates in an executable local
choice. Test whether this makes the retained palm-depth `0 -> 16` reversal
continue past `16` without extending a raw-tick window, inventing direction, or
adding state to the workstation adapter, then climb the unchanged world ladder
through surface contact, real key actuation, and release.

## Authority

- Path: `language.md`; `lessons.md` lessons 0a, 0b, 58 through 63; retained
  palm-depth evidence for link 2509 from consequence tick 424 to choice tick 429
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`

## Model

A live link is the existing physical arrow from participation to consequence.
Its consequence has two values: ordinary history, and an optional transition-
bearing outcome available for the first choice. Only a causal lineage containing
a real physical transition can fill the available value; ordinary samples can
update history but cannot create, refresh, or erase it. Candidate formation
projects the available tick through the existing completed-cycle field. The
first executable local competition containing that link consumes the
availability once, regardless of which alternative wins; historical consequence
time remains intact.

The new protocol inherits the complete behavior of
`RecursiveLearnerCausalTopologyProductComposition` and changes only organism
consequence lifetime. Learner-held construction outcomes retain their existing
lifecycle.

## Invariants

- Only existing consequence-recording paths carrying a real physical transition
  can make an outcome available.
- Availability is tied to an exact live link ID and generation; retirement and
  reuse clear it.
- Samples, blocked candidates, elapsed ticks, workstation poses, surfaces, and
  desired directions cannot create or refresh availability.
- The first executable local choice consumes every participating available
  outcome exactly once, while preserving `last_consequence_tick` as history.
- Ordinary protocols never create organism-held availability.
- Candidate formation, checkpoint restore, observation fingerprints, tracing,
  exact replay, work accounting, natural quiet, and evaluator isolation preserve
  the new state faithfully.
- The accepted production protocol and the existing four-tick ordinary rule do
  not change.

## Scope

- `truelearner/crates/core/src/core.rs`, `physics.rs`, `link.rs`, `learner.rs`,
  `outcome.rs`, `choose.rs`, `snapshot.rs`, and `trace.rs`
- `truelearner/crates/workstation/src/harness.rs`
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes production promotion, geometry changes, target knowledge, strength or
  opportunity changes, raw-tick extensions, held paths, and authority claims.

## Development style

TDD: first add core fixtures for creation, exact link-generation projection,
first-choice consumption, retirement/reuse, and checkpoint preservation. Then
require a transition-bearing outcome to be consumed by the next participating
choice and climb the unchanged real-world sequence until complete key release or
the first repeated session state.

## Focused tests

- Core unit tests prove ordinary protocols do not hold organism outcomes, the
  candidate protocol holds a real consequence across elapsed ticks, exact
  generation matching, one-use consumption, and retirement clearing.
- A core checkpoint fixture proves available state, consumption, replay, and
  observation fingerprint preservation across restore.
- The workstation fixture requires a consumed transition-bearing outcome and
  stops at complete key release or the first repeated full-session state.
- The exact contact witness is sequence 19, palm `(528, 768, 560)`, pressure
  `[0, 0, 8, 8, 8, 8]`, and exact one-step replay from the pre-contact checkpoint.
- The unchanged upward witness presses keys 42, 43, 80, and 81 at sequence 26,
  changes text to `]\\`, begins release at sequence 29, releases every key by
  sequence 61, and exactly replays the final release from its checkpoint.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves production behavior and replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable`
  preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its measured duration must remain strictly under 10 seconds. Candidate research
fixtures are timed separately and stop at their first falsifier.

## Controls and evidence

The held-out case is sustained palm motion through real keyboard contact, press,
and release. Negative controls are the parent protocol's exact tick-429 expiry,
ordinary samples, blocked-only moments, unrelated links, stale generations,
retired/reused links, a second choice, GenericOnly production, untraced runs,
replay, and the semantic firewall. Killing falsifiers are sample-created or
sample-refreshed availability, availability after the first participating
choice, a repeated full-session state before complete release, or any production
difference. Expected evidence is a lifecycle fixture, the retained parent
expiry, exact contact and release replay, and validated factory receipts.

## Risks and rollback

Consuming too early would reproduce the current failure; consuming too late
would let stale outcomes control later choices. Exact participating link
generations and consumption at the first executable group bound both risks.
Rollback removes the candidate protocol and link lifetime field; no workstation
or accepted-protocol behavior depends on them.

## Open decisions

None.
