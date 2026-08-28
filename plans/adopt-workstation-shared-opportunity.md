# Adopt one physical workstation opportunity

```text
one generic chance to move
          |
          v
one phase + one physical origin -> accepted learner -> one bounded movement

distinct real causes -> distinct origins -> accepted product -> composed movement
```

## Outcome

Make shared generic-opportunity incidence the normal `WorkstationHarness`
behavior after its positive frozen authority run. Keep the old independent
incidence available only behind the research feature so immutable parent
evidence remains reproducible.

This adopts a workstation boundary fact, not a new learner law. It establishes
neither intentional finger selection nor contact, pointing, clicking, typing,
grasping, or morphology transfer.

## Authority

- Path: `arch.md`; `academy.md` Body Discovery boundary;
  `research/campaigns/workstation-shared-opportunity-incidence-authority-v1/protocol.toml`;
  positive adjudication
  `research/campaigns/workstation-shared-opportunity-incidence-authority-v1/adjudication.toml`
- Revision: clean integration parent
  `f92249471fbcc35a7d5099972d32b201972a45e4`; adjudication SHA-256
  `c827e9b4b16807556ca947171549ae2c0e0d73416693b1156422e2cbdd2a46c8`

## Model

- The production state has one generic-opportunity incidence: all motor
  opportunities emitted by one external workstation step share arrival tick,
  phase, and physical origin. Targets remain distinct physical junctions.
- `WorkstationHarness::new` and `restore` therefore need no production mode or
  persisted selector. Their arrow is simply `WorldSample -> shared physical
  inputs -> accepted Harness -> WorkstationStepObservation`.
- Under the `research` feature only, retain the explicit sum type
  `ResearchOpportunityIncidence::{Independent, SharedWave}` so frozen parent
  and survivor evidence can still be generated. Keep that field compiled out
  of the production harness.
- Preserve the accepted `RecursiveLearnerCausalTopologyProductComposition`
  unchanged. Existing causal-origin choice handles one shared cause; topology
  product still composes distinct causes.
- Update the isolated experiment so its parent arm explicitly requests
  `Independent`, its shared arm explicitly requests `SharedWave`, and an
  adopted-default arm proves the normal constructor reproduces the authorized
  shared result.

## Invariants

- No production organism state, checkpoint field, protocol variant, learner
  selector, semantic body label, or evaluator fact is added.
- The initial default checkpoint bytes remain identical because topology,
  protocol, and empty pending state are unchanged.
- Default save/restore preserves the shared incidence arrow exactly.
- Research restore receives its incidence variant explicitly beside the opaque
  checkpoint; default restore never depends on research configuration.
- Independent research incidence reproduces the frozen forty-six five-finger
  parent wall. Shared research incidence and the adopted default reproduce ten
  isolated-finger steps across all five fingers and zero five-finger steps.
- Two genuinely distinct origins continue to compose.
- Exact replay, natural quiescence, semantic firewall, and maximum step work
  no greater than eight-hundred-twelve remain required.

## Scope

- Change incidence defaults and feature isolation in
  `truelearner/crates/workstation`.
- Update research-facing Academy restore signatures and adoption controls in
  `academy/crates/academy-workstation`.
- Update `research/experiments/workstation-digit-separation` with explicit
  historical modes and an adopted-default evidence arm.
- Add candidate and verification receipts under `factory/receipts/`.

Exclude core learner physics, workstation morphology, geometry, assets, force,
contact logic, device interpretation, checkpoint schema, video, authority
evidence mutation, and the next contact curriculum.

## Development style

TDD. First make the adopted-default evidence arm and production/reference
identity control fail against the independent default, then change the default
incidence and isolate the old mode behind the research feature.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core opportunity_origin`
  preserves shared-origin bounded choice and distinct-origin composition.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves checkpoint, body, receptor, and physical-transition controls.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --features research workstation_world`
  checks the real world, firewall, default/shared identity, and full morphology.
- `cargo test --locked --manifest-path research/experiments/workstation-digit-separation/Cargo.toml --lib`
  keeps the representative experiment suite fast.
- `cargo run --quiet --locked --manifest-path research/experiments/workstation-digit-separation/Cargo.toml -- --arm adopted-default`
  produces the full default 48-step replay evidence separately.
- `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check`
- `cargo clippy --workspace --all-targets --locked --manifest-path truelearner/Cargo.toml -- -D warnings`
- `cargo clippy --all-targets --locked --manifest-path research/experiments/workstation-digit-separation/Cargo.toml -- -D warnings`

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`

Its measured warm budget must remain strictly under 10 seconds. Full 48-step
adoption evidence is recorded separately.

## Controls and evidence

- Held-out cases: default versus explicit shared execution, restored default
  execution, explicit independent parent reproduction, and distinct origins.
- Negative controls: initial checkpoint identity, frozen parent counts, public
  sample semantic audit, corrupt checkpoint rejection, zero device events, and
  no core production diff.
- Falsifiers: default differs from explicit shared, any finger disappears, any
  five-finger step returns, replay differs, quiescence fails, work exceeds 812,
  independent history changes, or research mode enters the production struct.
- Evidence: validated plan; candidate receipt; adopted-default JSON; independent
  verification receipt; unchanged positive authority artifacts.

## Risks and rollback

- Accidentally changing historical parent reproduction would erase the
  necessity control. Detect it with the explicit independent arm.
- Serializing research mode would change checkpoint identity. Keep its field
  under `cfg(feature = "research")` and absent from checkpoint payloads.
- Treating separate movement as control would overstate the result. Keep the
  next contact curriculum outside this adoption.
- Roll back this adoption commit to restore the opt-in-only subject; the frozen
  authority evidence remains valid.

## Open decisions

None.
