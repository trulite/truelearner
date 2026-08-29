# Composable local spatial sensor fields

```text
physical field -> ordered finite product -> local product factorization -> learner wiring
       |                    |                         |
 distinct patterns     remain distinct       no semantic feature choice
```

## Outcome

Add one neutral spatial-field construction to `truelearner-embodiment` that can
carry an ordered finite field of available or unavailable sensor values and
losslessly factor it into local blocks. Exercise the same construction with the
real binocular workstation rasters retained at the monitor-cue failure. The new
projection must preserve the difference between the two learned glyph images
that the current point retina aliases, while identical images, uniform fields,
occlusion, locality, and exact reconstruction remain honest. This repairs and
tests only the arrow from physical image to spatial sensory value; it does not
yet wire thousands of cells into the learner or claim cue-driven key selection.

## Authority

- Path: `LANGUAGE.md`; `research/constitution.md`; `lessons.md` lessons 0, 0a,
  0b, 66, 69, 71, and 72;
  `research/campaigns/workstation-monitor-cue-retina-v1/convergence.toml`;
  `factory/receipts/workstation-monitor-cue-reuse-verification.json`;
  `plans/composable-active-sensing-effects.md`
- Revision: HEAD `dfe933886d4a030d7775356f78e908e8531c2fc2`;
  `LANGUAGE.md` SHA-256
  `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`;
  convergence SHA-256
  `46460a16fdf044aa1d31ea0ad7f135cc022720149eddcbfa98d463c9e4c913cf`;
  prior verification SHA-256
  `fff550f3d1999f30669c56223b18174eed9738a2f39d0acd061a1bd1a74e1718`

## Model

`SpatialField` is generic over a value type and a constant dimension count. It
is a finite ordered product with an explicit non-zero shape and exactly one
available-or-unavailable value at every row-major site. Construction checks
dimension, multiplication, and cell count before
admitting the field. It knows no image, eye, hand, key, target, or evaluator
noun. A two-dimensional light raster, depth raster, tactile sheet, and a held-out
three-dimensional field are instances of the same object.

`factor_local(block_shape)` consumes a field and reassociates that product into
ordered `LocalBlock`s. Every block retains its physical origin, actual edge
shape, and row-major values. Every source cell moves into exactly one block;
none is selected, copied, summarized, or dropped. `reassemble` consumes the
factorization and recovers the exact original field. Partial boundary blocks are
ordinary smaller products, not padded invented observations. Mapping a pure
value transformation before factorization equals mapping every local block
after factorization. `Unavailable` maps only to `Unavailable`.

In categorical terms, this is an isomorphism between one finite product and a
product of local finite products. It changes parentheses, not information.
Identity and composition therefore hold by construction, and distinct spatial
arrangements cannot be identified merely because they contain the same scalar
values. Existing `Signal`, `ChangeDetector`, `DriverBank`, and `Availability`
compose with a whole field or each local block without another driver trait.

The workstation adapter is research-only. It converts each already-visible
640-by-360 eye raster into one field and factors it into a fixed row-major grid
of 8-by-8 local blocks. The 8-by-8 morphology is frozen before inspecting the
cue pair, covers every pixel exactly once, and matches neither a glyph location
nor a key identity. The evaluator may compare retained blocks after projection;
no differing-block index is fed back to the organism.

## Invariants

- Shape and ordered site values are physical structure; field construction
  rejects zero dimensions, overflow, and wrong cell counts atomically.
- Availability is site-local. Occluded or absent values are never filled from a
  neighbor, a prior sample, or evaluator knowledge.
- Local factorization covers every input site exactly once and introduces no
  additional external input, semantic feature, selected target, or copied cue.
- Reassembly after factorization is exact, including non-divisible edge shapes.
- Pure mapping preserves shape and locality and commutes with factorization;
  identity and composition hold for fields and blocks.
- Two arrangements with the same value multiset remain different ordered
  fields; an unchanged arrangement remains an ordinary repeated sample.
- The construction is dimension- and value-generic and behaves unchanged for
  binocular intensity, depth with occlusion, tactile values, and a held-out
  three-dimensional field.
- The workstation projection uses the retained post-development gaze and real
  renderer. It does not move the cue, supply gaze, pick a distinguishing pixel,
  expose glyph identity, or change learner/body/world state.
- The previous point-retina counterexample, monitor-cue trace, real-key
  press/release behavior, stable fixation, Production behavior, semantic
  firewall, replay, natural quiescence, and bounded work remain unchanged.

## Scope

- `truelearner/crates/embodiment/src/lib.rs`: generic field, local block,
  factorization, reconstruction, mapping, and typed construction failures.
- `truelearner/crates/embodiment/tests/spatial_fields.rs`: categorical laws,
  boundaries, availability, multiple modalities, and held-out dimensions.
- `research/experiments/workstation-return-bearing-opportunity-composition/src/bin/`:
  retained real-raster local-field projection and evidence summary.
- `research/campaigns/workstation-monitor-cue-local-field-v1/`: frozen protocol,
  point-retina parent, local-field arm, artifact, and convergence.
- `factory/receipts/`: candidate and independent verification receipts.
- Excludes `truelearner-core`, accepted learner laws, workstation runtime retina,
  Academy world or renderer changes, semantic glyph features, convolution,
  salience, correspondence, attention, a held cue path, gaze routes, hand routes,
  learner wiring, Production adoption, Academy promotion, and capability claims.

## Development style

TDD. First write the field-law, invalid-construction, edge-block, arrangement,
occlusion, modality-transfer, and held-out-dimension tests. Implement the
smallest consuming product factorization that passes them. Only then compose it
with the retained real workstation raster and freeze the resulting projection.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment spatial_field`
  proves construction safety, identity, composition, factor/map commutation,
  lossless reassembly, edge locality, honest availability, arrangement
  distinction, and repeated-sample behavior.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment spatial_field_transfers_across_value_and_dimension`
  proves the same API composes intensity, occluded depth, tactile, and a held-out
  three-dimensional field without device branches.
- `cargo run --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --bin monitor_cue_local_field_projection -- research/campaigns/workstation-monitor-cue-reuse-v1/artifacts/monitor-cue-reuse.json research/campaigns/workstation-monitor-cue-local-field-v1/artifacts/local-field-projection.json`
  proves both real eye rasters reconstruct exactly and reports the first local
  block difference for the retained learned cue pair.
- `cargo run --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --bin monitor_cue_retina_projection -- research/campaigns/workstation-monitor-cue-reuse-v1/artifacts/monitor-cue-reuse.json /tmp/workstation-retina-projection.json`
  preserves the exact wider-point alias counterexample.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib monitor_cue_action_outcome_reuse_retains_the_first_broken_arrow`
  preserves the complete parent trace and its first-failure classifier.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation && cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserve runtime behavior and the semantic firewall.
- `cargo fmt --all -- --check`, `cargo check --locked`, and
  `cargo clippy --locked -- -D warnings` run in every affected workspace.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation`.
Its measured warm duration must remain strictly under 10 seconds; record cold
bootstrap separately.

## Controls and evidence

The held-out cases are a non-divisible 7-by-5 field, a three-dimensional field,
and the opposite eye raster. Negative controls are equal fields, two spatial
arrangements with the same multiset, a uniform field, unavailable interior and
edge cells, zero dimensions, wrong cell counts, overflow, repeated
factorization/reassembly, blank monitor cue, identical cue replay, and the
unchanged aliased point projection. Falsifiers are any lost, copied, reordered,
or invented site; failure of exact reconstruction; map/factor disagreement;
device-specific library vocabulary; a cue-specific block or offset; identical
local-field projections for different retained cue images; changed parent
evidence; semantic leakage; or a warm regression at or above ten seconds.

Evidence is a validated candidate receipt, independent verification receipt,
the immutable parent monitor-cue trace, the immutable point-alias projection,
and one compact local-field projection containing shapes, block origins,
per-eye reconstruction digests, differing-block counts, and exact replay source
digests. Raw duplicate raster or video artifacts are not added.

## Risks and rollback

A generic field can become an image framework, silently duplicate a large input,
or smuggle selected features into morphology. Keep only one field object, one
local block object, one consuming factorization, mapping, and exact
reconstruction. Bound checked allocation, move every value exactly once, keep
device adapters outside the shared crate, and freeze a complete regular tiling
before cue comparison. If laws, memory bounds, parent traces, or firewall checks
fail, remove the new embodiment types and projection while retaining the
falsified campaign; no checkpoint or persistence migration is involved.

## Open decisions

None.
