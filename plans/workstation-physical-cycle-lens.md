# Workstation physical CPU-cycle lens

```text
world state --sense--> body sample --physical transition--> next body --effect--> next world
                 |              |                    |
            environment     organism cost       observer evidence
```

## Outcome

Measure retired CPU cycles and instructions for the complete candidate's actual
physical transition separately from world rendering, checkpoint/replay,
fingerprinting, experiment projection, process startup, and test-harness work.
The result is a research diagnostic, not organism input or research authority.

## Authority

- Path: `truelearner/crates/workstation/src/harness.rs`,
  `academy/crates/academy-workstation/src/session.rs`, and the frozen complete
  candidate fixture in
  `research/experiments/workstation-return-bearing-opportunity-composition/src/runtime_attached_complete.rs`
- Revision: `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a` plus the candidate tree identified
  by `factory/receipts/workstation-runtime-attached-complete-candidate.json`

## Model

The measured objects are world state, admitted sample, body state, and world
effect. The arrows are sensing, physical transition, and effect application.
A research-only phase observer marks transaction cloning, core body runs,
diagnostic projection, and fingerprint construction without changing any arrow.
On macOS, a boundary adapter reads aggregate process cycles and instructions via
`proc_pid_rusage`; a release diagnostic composes those deltas additively and
reports environment, organism, body-core, and evidence costs. Unsupported hosts
fail explicitly rather than inventing a cycle estimate.

## Invariants

- Enabling phase observation produces the exact same next harness, public
  observation, canonical checkpoint, physical work, and natural quiescence.
- Host counters never enter body input, physical time, choice, checkpoint,
  fingerprint, replay comparison, or scientific predicates.
- Measurements run single-threaded so a process-wide counter delta belongs to
  the named arrow; the concurrent proof fixture remains unchanged.
- The cost of reading the host counter is measured separately and not presented
  as organism work.

## Scope

- Add a research-only phase observer to the workstation transition boundary.
- Add one release diagnostic binary that measures the same complete runtime-
  attached morphology with primary physical steps only.
- Add preservation and event-order tests.
- Exclude learner physics changes, renderer changes, checkpoint changes,
  authority execution, and performance optimization.

## Development style

TDD: first add the observer-preservation test, then implement the smallest
phase surface and diagnostic needed to make it pass.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --features research --test workstation_harness research_transition_phase_observer_preserves_the_physical_transition -- --exact`
  proves observer identity and ordered phase nesting.
- `cargo run --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --bin runtime_attached_physical_cycle_lens`
  emits the isolated cycle decomposition on macOS.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib runtime_attached_complete_candidate_retains_first_broken_arrow`
  proves the complete proof fixture remains unchanged and under 0.5 seconds.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --features research --test workstation_harness`.
It must complete under 10 seconds; cold compilation is recorded separately.

## Controls and evidence

- Held-out cases: a transition with no pending action return and a transition
  after an emitted movement, so both the main and returned core-body phases are
  observable.
- Negative controls: an empty adjacent counter read measures observer cost;
  an unobserved transition is the state-equality reference.
- Falsifiers: any state/checkpoint difference, counter data entering an
  observation, unbalanced phase events, concurrent work inside a measured
  region, or a nonzero body-core claim on an unsupported host.
- Expected artifact: one compact stdout report containing cycles, instructions,
  counts, and explicit inclusions/exclusions; no raw recording is committed.

## Risks and rollback

An observer placed across the wrong boundary would mislabel evidence work as
physical work. Phase-order tests and exact transition equality detect this;
rollback removes the observer surface and diagnostic binary without touching
organism state or protocols.

## Open decisions

None.
