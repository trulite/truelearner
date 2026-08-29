# Workstation embodiment wiring migration

```text
workstation geometry -> reusable wiring operations -> unchanged physical Harness
```

## Outcome

Move the workstation's complete receptor-bank, actuator-bank, projection,
outcome, anchor, link, and output-binding construction onto the neutral
`truelearner-embodiment` wiring API. Replace its private actuator-effort cells
with the shared opposed-effort type. Preserve exact junction/link insertion
order and the complete 64-step visual-reach trace byte for byte. This is a
behavior-preserving infrastructure refactor, not new sensorimotor capability.

## Authority

- Path: `arch.md` accepted body and boundaries; `LANGUAGE.md`; `algo.md`;
  `plans/composable-embodiment-drivers.md`; retained visual-reach trace
  `research/campaigns/workstation-return-bearing-opportunity-composition-v1/artifacts/visual-reach-64.json`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`; trace SHA-256
  `4be6c6f75aefe9dc2f38d019ce0f4f9133b26d650e0016a5b5b051f40241b2af`

## Model

`Wiring` is a thin effect boundary over the public physical `HarnessBuilder`.
Its objects are junction IDs and typed banks of junction IDs. Its arrows are
ordinary drive links and output-to-outcome bindings. Serial calls compose in
call order; independent banks compose by product; one source may fan out to
several targets while retaining one physical source.

The reusable API creates generic junctions, drive links, receptor banks,
actuator/output-sink banks, and outcome banks. It knows physical IDs, positions,
regions, thresholds, coupling, and bank shape, but no eye, hand, finger, key,
target, direction, score, or evaluator meaning. The workstation remains the
owner of geometry, body-axis grouping, retinotopic placement, relay selection,
and physical state integration.

`OpposedEffort` becomes the actuator frame's shared cell. Workstation direction
selection still accumulates bounded decrease/increase effort, and its existing
state law still converts net effort into the exact bounded body movement.

## Invariants

- The wrapper emits the same junctions, links, IDs, bindings, thresholds,
  regions, positions, coupling, and insertion order as direct construction.
- Workstation inputs, outputs, learner state, body state, checkpoints, physical
  time, diagnostics, work, and natural quiescence are unchanged.
- The retained 64-step visual-reach JSON has the exact pre-migration SHA-256.
- Receptor and actuator bank sizes are explicit and finite.
- One source fan-out adds internal links only; it never admits another external
  input or invents a transition.
- The embodiment crate contains no device or evaluator nouns and changes no
  learner law.
- Production/reference behavior, semantic firewall, checkpoint migration, and
  accepted hand behavior remain unchanged.

## Scope

- `truelearner/crates/embodiment/Cargo.toml`
- `truelearner/crates/embodiment/src/lib.rs`
- `truelearner/crates/embodiment/tests/`
- `truelearner/crates/workstation/src/harness.rs`
- `truelearner/crates/workstation/src/state.rs`
- affected Cargo lockfiles
- `factory/receipts/`
- Excludes core learner changes, workstation checkpoint schema changes, Academy
  behavior, new morphology, binocular alignment, hold outputs, and authority
  promotion.

## Development style

TDD. First compare a small direct `HarnessBuilder` construction with the same
construction through `Wiring`. Then migrate the workstation in insertion-order
chunks and require exact retained-trace equality after the complete refactor.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment wiring`
  proves direct/wrapped physical equality, bank order, fan-out, and bindings.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves workstation state, checkpoint, and replay behavior.
- `cargo run --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --bin visual_reach_trace -- /tmp/workstation-embodiment-after.json 64`
  followed by `shasum -a 256` requires exact trace digest
  `4be6c6f75aefe9dc2f38d019ce0f4f9133b26d650e0016a5b5b051f40241b2af`.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation`.
Its measured warm duration must remain strictly under 10 seconds; cold bootstrap
is recorded separately.

## Controls and evidence

The held-out control is the foveal 64-step trace and the non-research production
workstation suite. The identity control is that empty banks add no structure;
negative controls are reversed direct operation order, wrong bank arity,
checkpoint corruption, repeated retinal
samples, equal opposing effort, and the semantic firewall. Any trace-byte,
fingerprint, checkpoint, ID, work, or warm-budget difference falsifies the
refactor. Evidence is a validated candidate receipt and independent verification
receipt; no capability or authority evidence is produced.

## Risks and rollback

A bulk builder can reorder physical allocation or hide device meaning. Keep the
wrapper synchronous and call-order preserving, expose concrete bank-return
types, and leave all workstation placement callbacks in the workstation. If
exact trace equality fails, compare the first junction/link divergence and
revert only the wiring migration; persistence is unchanged.

## Open decisions

None.
